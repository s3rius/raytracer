use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    color::Color,
    interval::Interval,
    ppm::PPMImage,
    ray::Ray,
    scene::renderable::{RayData, Renderable},
    vec3::{Point3, Vec3},
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Camera {
    pub origin: Point3,

    pub focal_length: f32,
    pub aspect_ratio: f32,
    pub output_width: usize,

    output_height: usize,
    viewport_start: Point3,
    viewport_delta_h: Vec3,
    viewport_delta_w: Vec3,
}

fn ray_color(ray: &Ray) -> Color {
    let direction = ray.direction.normalize();
    let a = (direction.y + 1.) * 0.5;
    let v = Vec3::ONE * (1. - a) + (Vec3::new(0.5, 0.7, 1.0) * a);
    Color::from(v)
}

impl Camera {
    #[must_use]
    pub fn new(origin: Point3, aspect_ratio: f32, output_width: usize) -> Self {
        let mut output_height = (output_width as f32 / aspect_ratio) as usize;
        if output_height < 1 {
            output_height = 1;
        }
        let focal_length = 1.;
        let viewport_height = 2.;
        let viewport_width = viewport_height * (output_width as f32 / output_height as f32);
        let viewport_w = Vec3::new(viewport_width, 0., 0.);
        let viewport_h = Vec3::new(0., -viewport_height, 0.);

        let viewport_delta_w = viewport_w / output_width as f32;
        let viewport_delta_h = viewport_h / output_height as f32;

        let viewport_upper_left =
            origin - Vec3::new(0., 0., focal_length) - viewport_h / 2. - viewport_w / 2.;
        let viewport_start = viewport_upper_left + 0.5 * (viewport_delta_w + viewport_delta_h);

        Self {
            origin,
            focal_length,
            aspect_ratio,
            output_width,
            output_height,
            viewport_start,
            viewport_delta_h,
            viewport_delta_w,
        }
    }

    #[must_use]
    pub fn get_img(&self, scene: impl Renderable + Sync) -> PPMImage {
        let pixels = (0..self.output_height)
            .into_par_iter()
            .map(|y| {
                (0..self.output_width)
                    .into_par_iter()
                    .map(|x| {
                        let pixel_center = self.viewport_start
                            + (self.viewport_delta_w * x as f32)
                            + (self.viewport_delta_h * y as f32);
                        let ray_direction = pixel_center - self.origin;
                        let ray = Ray::new(self.origin, ray_direction);
                        let rd = RayData {
                            ray,
                            interval: Interval::new(0., f32::INFINITY),
                        };
                        if let Some(hit) = scene.hit(&rd) {
                            return Color::from((hit.normal + Vec3::ONE) / 2.);
                        }
                        ray_color(&ray)
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        pixels.into()
    }
}
