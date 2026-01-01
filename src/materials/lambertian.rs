use crate::{materials::MaterialRecord, ray::Ray, vec3::Vec3};

#[derive(Debug)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl super::Material for Lambertian {
    fn scatter(
        &self,
        _ray_in: &crate::ray::Ray,
        hit: &crate::renderables::HitRecord,
    ) -> Option<super::MaterialRecord> {
        let mut rng = rand::rng();
        let scattered_direction = hit.normal + Vec3::rand_unit(&mut rng);
        let scattered_ray = Ray::new(hit.point, scattered_direction);
        Some(MaterialRecord::new(self.albedo, scattered_ray))
    }
}
