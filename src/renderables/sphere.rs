use std::sync::Arc;

use crate::{
    materials::Material,
    renderables::{HitRecord, Renderable},
    vec3::Point3,
};

#[derive(Debug, Clone)]
pub struct Sphere {
    origin: Point3,
    radius: f32,
    material: Arc<dyn Material>,
}

impl Sphere {
    #[must_use]
    pub const fn new(origin: Point3, radius: f32, material: Arc<dyn Material>) -> Self {
        Self {
            origin,
            radius,
            material,
        }
    }
}
/// Sphere has this formula
///
/// x^2 + y^2 + z^2 = r^2
///
/// It defines all points on the surface of a sphere.
/// But it is true only for surface points. If we want to get
/// all the points inside of a sphere as well, then we need to
/// modify our formula and turn it into inequality.
///
/// x^2 + y^2 + z^2 <= r^2
///
/// That formula would be true for any point inside of the sphere
/// and surface points.
///
/// Now let's solve another problem with this formula. You might
/// noticed that it's true only for spheres located at 0,0,0.
/// Which is not true for most cases. But since we do rays here,
/// We can calculate this formula for any point of possible intersection instead.
///
/// Let me explain. Our main goal is to find out wether we need to render this object
/// or not. In order to do so, we need to see if ray anyhow intersects with the sphere.
/// And if it does, then inequality from the above should be true.
///
/// And where is going to be point of intersection? Well, of course
/// somewhere around origin< of a sphere. Let's define some symbols:
///
/// Q - Origin of a ray
/// D - direction of a ray
/// P - Point of intersection
/// P_0 - Sphere origin
///
/// The point of intersection will happen at P, which is in fact:
///
/// P = Q + D * t;
///
/// But does this point hits the sphere? Well, we can see it by
/// subtracting origin of a sphere from the point of intersection.
///
/// M = (P - P_0)
///
/// Why? To kinda move sphere to 0,0,0 coordinates. And for this
/// exact point the inequality should be true.  Let's substitute
///
/// M_x^2 + M_y^2 + M_z^2 <= r^2
///
/// But you might notice that we cannot calculate this inequality,
/// since we don't know the point of intersection. And that is true,
/// but we know all other variables. Let's subtitute again.
///
/// ((Q + D * t)_x - P_0_x)^2
/// + ((Q + D * t)_y - P_0_y)^2 +
/// + ((Q + D * t)_z - P_0_z)^2 <= r^2
///
/// But this formula looks like DOT product between two vectors.
/// Let's substitute.
///
/// (P - P_0) · (P - P_0) <= r^2
///
/// P · P - 2 * P · P_0 + P_0 · P_0 - r^2 <= 0
///
/// Let's substitute P:
///
///
/// (Q + dt) · (Q + dt) - 2 * (Q + dt) · P_0 + P_0 · P_0 - r^2 <= 0
///
/// Q · Q + 2 * dt · Q + dt^2 - 2 * P_0 · Q - 2 * P_0 · dt + P_0^2 <= 0
///
/// Let's transform it to the conventional quadratic formula:
///
/// dt^2 + t * (2d · Q - 2 * P_0 · d) + P_0^2 + Q^2 - 2 * P_0 · Q - r^2 <= 0
///
/// To solve it for `t` we need to find all roots:
///
/// a = d
/// b = 2d · Q - 2 * P_0 · d
/// c = P_0^2 + Q^2 - 2 * P_0 · Q - r^2
///
/// If root exist, then we're intersecting a sphere.
/// Also, for simplicity we can only render surface of the sphere,
/// which means just finding roots. But if we want to find all points,
/// we need to solve quadratic inequality.
impl Renderable for Sphere {
    fn hit(&self, ray: &super::RayData) -> Option<super::HitRecord> {
        let oc = self.origin - ray.ray.origin;
        let a = ray.ray.direction.length_squared();
        let h = ray.ray.direction.dot(oc);
        let c = self.radius.mul_add(-self.radius, oc.length_squared());
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
        Some(HitRecord::new_with_ray(
            &ray.ray,
            &point,
            &normal,
            root,
            self.material.clone(),
        ))
    }
}

impl Renderable for &Sphere {
    fn hit(&self, ray: &super::RayData) -> Option<super::HitRecord> {
        (*self).hit(ray)
    }
}
