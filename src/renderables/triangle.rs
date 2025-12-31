use crate::{
    renderables::Renderable,
    vec3::{Point3, Vec3},
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Triangle {
    a: Point3,
    b: Point3,
    c: Point3,
}

impl Triangle {
    pub fn new(a: Point3, b: Point3, c: Point3) -> Self {
        Self { a, b, c }
    }
    pub fn get_normal(&self) -> Vec3 {
        self.a.cross(&self.b)
    }
}

impl Renderable for Triangle {
    fn hit(&self, ray: &super::RayData) -> Option<super::HitRecord> {
        let whatever = self.a + self.b - self.c;
        let a = ray.ray.direction.len_squared();
        let b = ray.ray.origin.dot(ray.ray.direction) - ray.ray.direction.dot(whatever);
        let c = ray.ray.origin.len_squared() - ray.ray.origin.dot(whatever)
            + self.a.len_squared()
            + self.b.len_squared()
            - self.c.len_squared();

        let discriminant = b.powi(2) - 4. * a * c;
        if discriminant < 0. {
            return None;
        }

        let sqrd = discriminant.sqrt();
        println!("Found root");

        let mut root = (-b - sqrd) / (2. * a);
        if !ray.interval.contains(root) {
            root = (-b + sqrd) / (2. * a);
        }

        let point = ray.ray.at(root);
        let normal = self.get_normal();
        Some(super::HitRecord::new_with_ray(
            &ray.ray, &point, &normal, root,
        ))
    }
}
