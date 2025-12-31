use std::fmt::Debug;

use crate::{
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};
mod scene;
mod sphere;

pub use scene::Scene;
pub use sphere::Sphere;

pub trait Renderable: Debug {
    fn hit(&self, ray: &RayData) -> Option<HitRecord>;
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub distance: f32,
    pub front_face: bool,
}

impl HitRecord {
    #[must_use]
    pub fn new_with_ray(ray: Ray, point: Point3, normal: Vec3, distance: f32) -> Self {
        let mut record = Self {
            point,
            normal,
            distance,
            front_face: false,
        };
        record.front_face = ray.direction.dot(normal) < 0.;
        if !record.front_face {
            record.normal = -normal;
        }
        record
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct RayData {
    pub ray: Ray,
    pub interval: Interval,
}
