use crate::{color::Color, ray::Ray, scene::renderable::Renderable, vec3::Point3};

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
        let a = ray.direction.dot(&ray.direction);
        let b = (ray.direction * -2.).dot(&oc);
        let c = oc.dot(&oc) - self.radius.powi(2);
        let root = b.powi(2) - 4. * a * c;
        if root < 0. {
            return None;
        }
        Some(Color::RED)
    }
}
