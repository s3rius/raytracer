use crate::{
    color::Color,
    ray::Ray,
    scene::renderable::Renderable,
    vec3::{Point3, Vec3},
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
    fn render(&self, ray: &Ray) -> Option<Color> {
        let oc = self.origin - ray.origin;
        let a = ray.direction.len_squared();
        let h = ray.direction.dot(&oc);
        let c = oc.len_squared() - self.radius.powi(2);
        let discriminant = h.powi(2) - a * c;
        if discriminant < 0. {
            return None;
        }
        let root = (h - discriminant.sqrt()) / a;
        let norm = (ray.at(root) + Vec3::new(0., 0., 1.)).normalize();
        Some(Color::from((norm + Vec3::ONE) / 2.))
    }
}
