use crate::{
    materials::Material,
    ray::Ray,
    vec3::{Vec3, Vec3Ext},
};

#[derive(Debug, Clone)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo, fuzz: 1. }
    }

    pub fn with_fuzz(mut self, fuzz: f32) -> Self {
        if fuzz > 1.0 {
            self.fuzz = 1.0;
        } else {
            self.fuzz = fuzz;
        }
        self
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &crate::ray::Ray,
        hit: &crate::renderables::HitRecord,
    ) -> Option<super::MaterialRecord> {
        let mut rng = rand::rng();
        let mut reflection = ray_in.direction.reflect(hit.normal);
        reflection = reflection.normalize() + (self.fuzz * Vec3::rand_unit(&mut rng));
        if reflection.dot(hit.normal) > 0. {
            Some(super::MaterialRecord::new(
                self.albedo,
                Ray::new(hit.point, reflection),
            ))
        } else {
            None
        }
    }
}
