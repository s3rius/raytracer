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
    #[must_use]
    pub const fn new(origin: Point3, normal: Vec3) -> Self {
        Self { origin, normal }
    }
}

/// Plane is simple to render.
///
/// Q - Origin of a ray
/// D - direction of a ray
/// P - Point of intersection
/// P_0 - Plane's origin
/// N - Plane's normal
///
/// Since we know that
/// Dot product of 2 perpendicular lines
/// is equal to 0,
/// then we might think that if a dot product of a normal with a vector that goes from
/// the point of intersection to to the origin of the origin of the plain equals 0,
/// then it's somewhere on the plane.
///
/// It's described by the following equasion:
///
/// (P - P_0) · N = 0
///
/// Our Point of Intesection (P) is described by the formula:
/// P = Q + D * t
///
/// We don't know t, but we know all other parameters. Let's substitue
/// and find the resulting formula for t.
///
/// (Q + D * t - P_0) · N = 0
/// -> Q · N + ( D * t ) · N - P_0 · N = 0
/// -> t = ((P_0 - Q) · N) / (d · N)
///
/// In case if t is less than zero, it means that 
/// there's no intersection.
///
impl Renderable for Plane {
    fn hit(&self, ray: &super::RayData) -> Option<super::HitRecord> {
        let t =
            (self.origin - ray.ray.origin).dot(self.normal) / ray.ray.direction.dot(self.normal);
        if t < 0. || !ray.interval.contains(t) {
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
