use std::sync::Arc;

use crate::{
    materials::Material,
    renderables::{Plane, Renderable},
    vec3::{Point3, Vec3},
};

#[derive(Debug, Clone)]
pub struct Triangle {
    a: Point3,
    b: Point3,
    c: Point3,

    // Barecentric coordinates.
    // Used for plane intersecting.
    plane: Plane,
    u: Vec3,
    v: Vec3,
    // Normal
    n: Vec3,
}

impl Triangle {
    #[must_use]
    pub fn new(a: Point3, b: Point3, c: Point3, material: Arc<dyn Material>) -> Self {
        // We pre-calculate triangle's plane it lies on.
        let n = (b - a).cross(c - a);
        let plane = Plane::new((a + b + c) / 3, n, material);
        Self {
            a,
            b,
            c,
            plane,
            u: b - a,
            v: c - a,
            n,
        }
    }

    #[must_use]
    pub const fn a(&self) -> Point3 {
        self.a
    }

    #[must_use]
    pub const fn b(&self) -> Point3 {
        self.b
    }

    #[must_use]
    pub const fn c(&self) -> Point3 {
        self.c
    }
}

/// <https://math.stackexchange.com/questions/544946/determine-if-projection-of-3d-point-onto-plane-is-within-a-triangle>
///
/// Let u⃗ = B - A, v⃗ = C - A, n⃗ = u⃗ × v⃗ , w⃗ = P − A.
///
/// We then have directly the barycentric coordinates of the projection P′ of P onto T as
/// γ=(u⃗×w⃗)⋅n⃗/∥n⃗∥2
/// β=(w⃗×v⃗)⋅n⃗/∥n⃗∥2
/// α=1−γ−β
///
/// todo!("Rewrite with Möller-Trumbore algorithm")
impl Renderable for Triangle {
    fn hit(&self, ray: &super::RayData) -> Option<super::HitRecord> {
        let hit = self.plane.hit(ray)?;
        let w = hit.point - self.a;
        let n_len = self.n.len_squared();

        let gamma = self.u.cross(w).dot(self.n) / n_len;
        let betta = w.cross(self.v).dot(self.n) / n_len;
        let alpha = 1. - gamma - betta;
        let range = 0.0..=1.0;

        if range.contains(&alpha) && range.contains(&betta) && range.contains(&gamma) {
            return Some(hit);
        }
        None
    }
}
