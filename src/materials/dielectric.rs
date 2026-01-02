use glam::Vec3;
use rand::Rng;

use crate::ray::Ray;

#[derive(Debug)]
pub struct Dielectric {
    pub refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Self {
        Self { refraction_index }
    }
}

#[inline]
/// Schlick's approximation for reflectance.
fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
    let r0 = ((1. - refraction_index) / (1. + refraction_index)).powi(2);
    r0 + (1. - r0) * (1. - cosine).powi(5)
}

impl super::Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &crate::ray::Ray,
        hit: &crate::renderables::HitRecord,
    ) -> Option<super::MaterialRecord> {
        let attenutation = Vec3::ONE;
        let ri = if hit.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = ray_in.direction.normalize();
        let cos_theta = hit.normal.dot(-unit_direction).min(1.0);
        let sin_theta = (1. - cos_theta.powi(2)).sqrt();
        let mut rng = rand::rng();

        // We cannot refract, because there's no
        // solution for snell's law for this ray.
        // Falling back to reflections.
        let direction =
            if ri * sin_theta > 1.0 || reflectance(cos_theta, ri) > rng.random_range(0.0..1.0) {
                unit_direction.reflect(hit.normal)
            } else {
                unit_direction.refract(hit.normal, ri)
            };
        Some(super::MaterialRecord::new(
            attenutation,
            Ray::new(hit.point, direction),
        ))
    }
}
