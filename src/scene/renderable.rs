use std::fmt::Debug;

use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct RayData {
    pub ray: Ray,
    pub t_min: f32,
    pub t_max: f32,
}

pub trait Renderable: Debug {
    fn hit(&self, ray: &RayData) -> Option<HitRecord>;
}
