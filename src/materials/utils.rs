use crate::{ray::Ray, vec3::Vec3};

pub struct MaterialRecord {
    pub attenuation: Vec3,
    pub ray: Ray,
}

impl MaterialRecord {
    pub fn new(attenuation: Vec3, ray: Ray) -> Self {
        Self { attenuation, ray }
    }
}
