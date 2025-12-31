use crate::{
    renderables::{HitRecord, Renderable},
    vec3::Point3,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Sphere {
    origin: Point3,
    radius: f32,
}

impl Sphere {
    #[must_use]
    pub const fn new(origin: Point3, radius: f32) -> Self {
        Self { origin, radius }
    }
}
/// Sphere has formula
///
/// x^2 + y^2 + z^2 = r^2
///
/// Therefore we can solve this
/// equasion for certain points to
/// find out if ray hits the sphere.
///
/// (Origin-Ray) is the vector that goes from
/// ray to the center of the sphere.
///
/// If the equasion has solution, then we hit the
/// sphere.
impl Renderable for Sphere {
    fn hit(&self, ray: &super::RayData) -> Option<super::HitRecord> {
        let oc = self.origin - ray.ray.origin;
        let a = ray.ray.direction.len_squared();
        let h = ray.ray.direction.dot(oc);
        let c = self.radius.mul_add(-self.radius, oc.len_squared());
        let discriminant = h.mul_add(h, -(a * c));
        if discriminant < 0. {
            return None;
        }
        let dsqrt = discriminant.sqrt();
        // square equasion has 2 solutions.
        // Checking both.
        let mut root = (h - dsqrt) / a;
        if !ray.interval.contains(root) {
            root = (h + dsqrt) / a;
            // Second solution also doesn't work.
            if !ray.interval.contains(root) {
                return None;
            }
        }
        let point = ray.ray.at(root);
        let normal = (point - self.origin) / self.radius;
        Some(HitRecord::new_with_ray(&ray.ray, &point, &normal, root))
    }
}

impl Renderable for &Sphere {
    fn hit(&self, ray: &super::RayData) -> Option<super::HitRecord> {
        (*self).hit(ray)
    }
}
