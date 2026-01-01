use std::fmt::Debug;

use crate::{materials::MaterialRecord, ray::Ray, renderables::HitRecord};

pub trait Material: Debug + Send + Sync {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<MaterialRecord>;
}
