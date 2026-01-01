use std::{fmt::Debug, sync::Arc};

use crate::{
    interval::Interval,
    materials::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Debug, Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub distance: f32,
    pub front_face: bool,
    pub material_ref: Arc<dyn Material>,
}

impl HitRecord {
    #[must_use]
    pub fn new_with_ray(
        ray: &Ray,
        point: &Point3,
        normal: &Vec3,
        distance: f32,
        material_ref: Arc<dyn Material>,
    ) -> Self {
        let mut record = Self {
            point: *point,
            normal: *normal,
            distance,
            front_face: false,
            material_ref,
        };
        record.front_face = ray.direction.dot(*normal) < 0.;
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
