use crate::{
    scene::renderable::{HitRecord, RayData, Renderable},
    vec3::Point3,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Sphere {
    origin: Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(origin: Point3, radius: f32) -> Self {
        Self { origin, radius }
    }
}

impl Renderable for Sphere {
    fn hit(&self, ray: &RayData) -> Option<HitRecord> {
        let oc = self.origin - ray.ray.origin;
        let a = ray.ray.direction.len_squared();
        let h = ray.ray.direction.dot(&oc);
        let c = oc.len_squared() - self.radius.powi(2);
        let discriminant = h.powi(2) - a * c;
        if discriminant < 0. {
            return None;
        }
        let dsqrt = discriminant.sqrt();
        // square equasion has 2 solutions.
        // Checking both.
        let mut root = (h - dsqrt) / a;
        if ray.t_min >= root || ray.t_max <= root {
            root = (h + dsqrt) / a;
            // Second solution also doesn't work.
            if ray.t_min >= root || ray.t_max <= root {
                return None;
            }
        }
        let point = ray.ray.at(root);
        let normal = (point - self.origin) / self.radius;
        Some(HitRecord {
            t: root,
            normal,
            point,
        })
    }
}
