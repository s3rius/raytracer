use crate::vec3::{Point3, Vec3};

pub struct Camera {
    pub origin: Point3,
    pub direction: Vec3,

    pub output_width: usize,
    pub output_height: usize,
}

impl Camera {
    pub fn new(origin: Point3, direction: Vec3, output_width: usize, output_height: usize) -> Self {
        Self {
            origin,
            direction,
            output_width,
            output_height,
        }
    }

    pub fn cast_rays() -> impl Iterator {
        0..2
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            origin: Point3::ZERO,
            direction: Vec3::ZERO.with_z(1.),
            output_width: 1280,
            output_height: 720,
        }
    }
}
