use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    color::Color,
    ppm::PPMImage,
    ray::Ray,
    scene::{renderable::RayData, scene::Scene},
    vec3::{Point3, Vec3},
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Camera {
    pub origin: Point3,

    pub focal_length: f32,
    pub output_width: usize,
    pub output_height: usize,
}

fn ray_color(ray: &Ray) -> Color {
    let direction = ray.direction.normalize();
    let a = (direction.y + 1.) * 0.5;
    let v = Vec3::ONE * (1. - a) + (Vec3::new(0.5, 0.7, 1.0) * a);
    Color::from(v)
}

impl Camera {
    #[must_use]
    pub const fn new(origin: Point3, output_width: usize, output_height: usize) -> Self {
        Self {
            origin,
            output_width,
            output_height,
            focal_length: 1.0,
        }
    }

    #[must_use]
    pub fn get_img(&self, scene: &Scene) -> PPMImage {
        let viewport_height = 2.;
        let viewport_width =
            viewport_height * (self.output_width as f32 / self.output_height as f32);
        let viewport_u = Vec3::new(viewport_width, 0., 0.);
        let viewport_v = Vec3::new(0., -viewport_height, 0.);

        let pixel_delta_u = viewport_u / self.output_width as f32;
        let pixel_delta_v = viewport_v / self.output_height as f32;

        let view_port_upper_left =
            self.origin - Vec3::new(0., 0., self.focal_length) - viewport_u / 2. - viewport_v / 2.;
        let pixel00_loc = view_port_upper_left + 0.5 * (pixel_delta_v + pixel_delta_u);
        let pixels = (0..self.output_height)
            .into_par_iter()
            .map(|y| {
                (0..self.output_width)
                    .into_par_iter()
                    .map(|x| {
                        let pixel_center =
                            pixel00_loc + (pixel_delta_u * x as f32) + (pixel_delta_v * y as f32);
                        let ray_direction = pixel_center - self.origin;
                        let ray = Ray::new(self.origin, ray_direction);
                        let rd = RayData {
                            ray,
                            t_min: 0.,
                            t_max: f32::INFINITY,
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

impl Default for Camera {
    fn default() -> Self {
        Self {
            origin: Point3::ZERO,
            output_width: 1270,
            output_height: 720,
            focal_length: 1.0,
        }
    }
}
