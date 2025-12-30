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
    pub direction: Vec3,

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
    pub fn new(origin: Point3, direction: Vec3, output_width: usize, output_height: usize) -> Self {
        Self {
            origin,
            direction,
            output_width,
            output_height,
            focal_length: 1.0,
        }
    }

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
                        let ray = Ray::new(pixel_center, ray_direction);
                        let rd = RayData {
                            ray: ray,
                            t_min: -10.,
                            t_max: 10.,
                        };
                        scene.hit(&rd).unwrap_or_else(|| ray_color(&ray))
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
            direction: Vec3::ZERO.with_z(1.),
            output_width: 1270,
            output_height: 720,
            focal_length: 1.0,
        }
    }
}
