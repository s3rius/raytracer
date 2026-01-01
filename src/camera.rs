use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    color::Color,
    interval::Interval,
    ppm::PPMImage,
    ray::Ray,
    renderables::{RayData, Renderable},
    vec3::{Point3, Vec3},
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Camera {
    pub origin: Point3,
    pub aspect_ratio: f32,
    pub output_width: usize,
    pub anti_aliasing_samples: usize,
    pub max_depth: usize,
    anti_aliasing_scale: f32,

    focal_length: f32,
    output_height: usize,
    viewport_start: Point3,
    viewport_delta_h: Vec3,
    viewport_delta_w: Vec3,
}

fn get_color_vec(
    ray: Ray,
    depth: usize,
    scene: &impl Renderable,
    rng: &mut impl rand::Rng,
) -> Vec3 {
    if depth == 0 {
        return Vec3::ZERO;
    }

    let rd = RayData {
        ray,
        interval: Interval::new(0., f32::INFINITY),
    };

    if let Some(hit) = scene.hit(&rd) {
        let random_on_hemisphere = Vec3::rand_on_hemisphere(rng, hit.normal);
        return 0.5
            * get_color_vec(
                Ray::new(hit.point, random_on_hemisphere),
                depth - 1,
                scene,
                rng,
            );
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
            anti_aliasing_samples: 1,
            anti_aliasing_scale: 1.,
            max_depth: 100,
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
        let viewport_height = 2.;
        let viewport_width =
            viewport_height * (self.output_width as f32 / self.output_height as f32);
        let viewport_w = Vec3::new(viewport_width, 0., 0.);
        let viewport_h = Vec3::new(0., -viewport_height, 0.);

        let viewport_delta_w = viewport_w / self.output_width as f32;
        let viewport_delta_h = viewport_h / self.output_height as f32;

        let viewport_upper_left =
            self.origin - Vec3::new(0., 0., focal_length) - viewport_h / 2. - viewport_w / 2.;
        let viewport_start = viewport_upper_left + 0.5 * (viewport_delta_w + viewport_delta_h);

        self.focal_length = focal_length;
        self.viewport_start = viewport_start;
        self.viewport_delta_h = viewport_delta_h;
        self.viewport_delta_w = viewport_delta_w;

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
    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }

    #[must_use]
    pub fn get_img(&self, scene: &(impl Renderable + Sync)) -> PPMImage {
        let pixels = (0..self.output_height)
            .into_par_iter()
            .map(|y| {
                (0..self.output_width)
                    .into_par_iter()
                    .map(|x| {
                        let mut rng = rand::rng();
                        rng.reseed().ok();
                        if self.anti_aliasing_samples == 0 {
                            self.get_color_simple(x, y, scene, &mut rng)
                        } else {
                            self.get_color_antialiased(x, y, scene, &mut rng)
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        pixels.into()
    }

    fn get_color_simple(
        &self,
        x: usize,
        y: usize,
        scene: &impl Renderable,
        rng: &mut impl rand::Rng,
    ) -> Color {
        let pixel_center = self.viewport_start
            + (self.viewport_delta_w * x as f32)
            + (self.viewport_delta_h * y as f32);
        let ray_direction = pixel_center - self.origin;
        let ray = Ray::new(self.origin, ray_direction);
        Color::from(get_color_vec(ray, self.max_depth, scene, rng))
    }

    fn get_color_antialiased(
        &self,
        x: usize,
        y: usize,
        scene: &impl Renderable,
        rng: &mut impl rand::Rng,
    ) -> Color {
        let mut color_vec = Vec3::ZERO;
        for _ in 0..self.anti_aliasing_samples {
            let offset_x: f32 = rng.random_range(-0.5..=0.5);
            let offset_y: f32 = rng.random_range(-0.5..=0.5);
            let pixel_center = self.viewport_start
                + (self.viewport_delta_w * (x as f32 + offset_x))
                + (self.viewport_delta_h * (y as f32 + offset_y));
            let ray_direction = pixel_center - self.origin;
            let ray = Ray::new(self.origin, ray_direction);
            color_vec += get_color_vec(ray, self.max_depth, scene, rng);
        }

        Color::from(color_vec * self.anti_aliasing_scale)
    }
}
