use crate::{
    renderables::Renderable,
    vec3::{Point3, Vec3},
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Plane {
    pub origin: Point3,
    pub normal: Vec3,
}

impl Plane {
    pub fn new(origin: Point3, normal: Vec3) -> Self {
        Self { origin, normal }
    }
}

impl Renderable for Plane {
    fn hit(&self, ray: &super::RayData) -> Option<super::HitRecord> {
        let t =
            (self.origin - ray.ray.origin).dot(self.normal) / ray.ray.direction.dot(self.normal);
        if t < 0. || !ray.interval.contains(t)  {
            return None;
        }
        let point = ray.ray.at(t);
        Some(super::HitRecord::new_with_ray(
            &ray.ray,
            &point,
            &self.normal,
            t,
        ))
    }
}
