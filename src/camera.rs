use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    color::Color,
    interval::Interval,
    ppm::PPMImage,
    ray::Ray,
    renderables::{RayData, Renderable},
    vec3::{Point3, Vec3, Vec3Ext},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    pub origin: Point3,
    pub lookat: Point3,
    pub aspect_ratio: f32,
    pub output_width: usize,
    pub anti_aliasing_samples: usize,
    pub max_depth: usize,
    pub fov: usize,
    anti_aliasing_scale: f32,

    focal_length: f32,
    output_height: usize,
    viewport_start: Point3,
    viewport_delta_h: Vec3,
    viewport_delta_w: Vec3,
}

fn get_color_vec(ray: Ray, depth: usize, scene: &impl Renderable) -> Vec3 {
    if depth == 0 {
        return Vec3::ZERO;
    }

    let rd = RayData {
        ray,
        interval: Interval::new(0.001, f32::INFINITY),
    };

    if let Some(hit) = scene.hit(&rd) {
        if let Some(mat_record) = hit.material_ref.scatter(&rd.ray, &hit) {
            return mat_record.attenuation * get_color_vec(mat_record.ray, depth - 1, scene);
        }
        return Vec3::ZERO;
    }

    let direction = ray.direction.normalize();
    let a = (direction.y + 1.) * 0.5;
    Vec3::ONE * (1. - a) + a * Vec3::new(0.5, 0.7, 1.0)
}

impl Camera {
    #[must_use]
    pub fn new(origin: Point3, aspect_ratio: f32, output_width: usize) -> Self {
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let mut output_height = (output_width as f32 / aspect_ratio) as usize;
        if output_height < 1 {
            output_height = 1;
        }
        let focal_length = 1.;

        Self {
            origin,
            focal_length,
            lookat: Vec3::new(0., 0., -1.),
            anti_aliasing_samples: 1,
            anti_aliasing_scale: 1.,
            max_depth: 100,
            fov: 90,
            aspect_ratio,
            output_width,
            output_height,
            viewport_start: Vec3::ZERO,
            viewport_delta_h: Vec3::ZERO,
            viewport_delta_w: Vec3::ZERO,
        }
        .with_focal_length(1.)
        .with_anti_aliasing_samples(10)
    }

    #[must_use]
    pub fn with_focal_length(mut self, focal_length: f32) -> Self {
        self.focal_length = focal_length;
        // Fov angle.
        let theta = (self.fov as f32).to_radians();
        // Since FOV is a whole angle, we want to calculate
        // only upper half to see height of a viewport.
        //
        // Since tan = oppose / adjasent, we can say that
        // oppose side of the triangle would be equal to
        // tg(Θ) * adjacent. Which is equal to focal len.
        let viewport_height = (0.5 * theta).tan() * focal_length * 2.;
        // Now when we know our height, we can calculate width of a viewport.
        let viewport_width = viewport_height * self.aspect_ratio;

        // Vector that points from origin to lookat
        let viewport_forward = (self.lookat - self.origin).normalize();
        // Vector that points to the right on the viewport (perpendicular to UP and forward
        // direction);
        let viewport_right = Vec3::UP.cross(viewport_forward).normalize();
        // Vector that points up on the viewport (perpendicular to forward and right).
        let viewport_up = viewport_forward.cross(viewport_right).normalize();

        self.viewport_start =
            self.origin + viewport_forward * focal_length + (viewport_up * 0.5 * viewport_height)
                - (0.5 * viewport_width * viewport_right);

        self.viewport_delta_h = -viewport_up * viewport_height / self.output_height as f32;
        self.viewport_delta_w = viewport_right * viewport_width / self.output_width as f32;

        self
    }

    #[must_use]
    pub fn reinit(mut self) -> Self {
        let focal_len = (self.lookat - self.origin).length();
        self = self.with_focal_length(focal_len);
        self
    }

    #[must_use]
    pub fn with_anti_aliasing_samples(mut self, samples: usize) -> Self {
        self.anti_aliasing_samples = samples;
        if samples != 0 {
            self.anti_aliasing_scale = 1. / self.anti_aliasing_samples as f32;
        }
        self
    }

    #[must_use]
    pub const fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }

    #[must_use]
    pub fn with_fov(mut self, fov: usize) -> Self {
        self.fov = fov;
        self.reinit()
    }

    #[must_use]
    pub fn with_lookat(mut self, lookat: Point3) -> Self {
        self.lookat = lookat;
        self.reinit()
    }

    #[must_use]
    pub fn get_img(&self, scene: &(impl Renderable + Sync)) -> PPMImage {
        let pixels = (0..self.output_height)
            .into_par_iter()
            .map(|y| {
                (0..self.output_width)
                    .into_par_iter()
                    .map(|x| {
                        if self.anti_aliasing_samples == 0 {
                            self.get_color_simple(x, y, scene)
                        } else {
                            self.get_color_antialiased(x, y, scene)
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        pixels.into()
    }

    fn get_color_simple(&self, x: usize, y: usize, scene: &impl Renderable) -> Color {
        let pixel_center = self.viewport_start
            + (self.viewport_delta_w * x as f32)
            + (self.viewport_delta_h * y as f32);
        let ray_direction = pixel_center - self.origin;
        let ray = Ray::new(self.origin, ray_direction);
        Color::from(get_color_vec(ray, self.max_depth, scene))
    }

    fn get_color_antialiased(&self, x: usize, y: usize, scene: &impl Renderable) -> Color {
        let mut rng = rand::rng();
        let mut color_vec = Vec3::ZERO;
        for _ in 0..self.anti_aliasing_samples {
            let offset_x: f32 = rng.random_range(-0.5..=0.5);
            let offset_y: f32 = rng.random_range(-0.5..=0.5);
            let pixel_center = self.viewport_start
                + (self.viewport_delta_w * (x as f32 + offset_x))
                + (self.viewport_delta_h * (y as f32 + offset_y));
            let ray_direction = pixel_center - self.origin;
            let ray = Ray::new(self.origin, ray_direction);
            color_vec += get_color_vec(ray, self.max_depth, scene);
        }

        Color::from(color_vec * self.anti_aliasing_scale)
    }
}
