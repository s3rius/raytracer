use std::sync::Arc;

use crate::{
    materials::Material,
    renderables::{HitRecord, Renderable},
    vec3::Point3,
};

#[derive(Debug, Clone)]
pub struct Triangle {
    pub a: Point3,
    pub b: Point3,
    pub c: Point3,
    pub material: Arc<dyn Material>,
}

impl Triangle {
    #[must_use]
    pub fn new(a: Point3, b: Point3, c: Point3, material: Arc<dyn Material>) -> Self {
        Self { a, b, c, material }
    }

    pub fn move_to(&mut self, point: Point3) {
        let centroid = (self.a + self.b + self.c) / 3.;
        self.a = point + (self.a - centroid);
        self.b = point + (self.b - centroid);
        self.c = point + (self.c - centroid);
    }

    pub fn rotate(&mut self, quat: glam::Quat) {
        self.a = quat * self.a;
        self.b = quat * self.b;
        self.c = quat * self.c;
    }
}

/// Honestly. It's too complicated.
/// But it's the fastest way to calculate it.
///
/// Google for Möller-Trumbore algorithm
impl Renderable for Triangle {
    fn hit(&self, ray: &super::RayData) -> Option<super::HitRecord> {
        let edge1 = self.b - self.a;
        let edge2 = self.c - self.a;
        let h = ray.ray.direction.cross(edge2);
        let det = edge1.dot(h);

        if det.abs() < f32::EPSILON {
            // The ray is parallel to triangle.
            return None;
        }

        let inv_det = 1.0 / det;
        let s = ray.ray.origin - self.a;
        let u = inv_det * s.dot(h);

        if (u < 0.0 && u.abs() > f32::EPSILON) || (u > 1.0 && (u - 1.).abs() > f32::EPSILON) {
            return None;
        }

        let q = s.cross(edge1);
        let v = inv_det * ray.ray.direction.dot(q);
        if (v < 0.0 && v.abs() > f32::EPSILON) || (u + v > 1.0 && (u + v - 1.).abs() > f32::EPSILON)
        {
            return None;
        }

        let t = inv_det * edge2.dot(q);

        if !ray.interval.contains(t) || t <= f32::EPSILON {
            return None;
        }

        let point = ray.ray.at(t);
        let normal = (edge1.cross(edge2) - self.a).normalize();
        Some(HitRecord::new_with_ray(
            &ray.ray,
            &point,
            &normal,
            t,
            self.material.clone(),
        ))
    }
}
