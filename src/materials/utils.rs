use crate::{ray::Ray, vec3::Vec3};

pub struct MaterialRecord {
    pub attenuation: Vec3,
    pub ray: Ray,
}

impl MaterialRecord {
    #[must_use] 
    pub const fn new(attenuation: Vec3, ray: Ray) -> Self {
        Self { attenuation, ray }
    }
}
