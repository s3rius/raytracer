use crate::{materials::Material, ray::Ray, vec3::Vec3};

#[derive(Debug, Clone)]
pub struct Metal {
    pub albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &crate::ray::Ray,
        hit: &crate::renderables::HitRecord,
    ) -> Option<super::MaterialRecord> {
        let scattered = ray_in.direction.reflect(hit.normal);
        Some(super::MaterialRecord::new(
            self.albedo,
            Ray::new(hit.point, scattered),
        ))
    }
}
