use std::fmt::Debug;

use crate::{color::Color, ray::Ray, renderables::HitRecord};

pub trait Material: Debug {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord, attenuation: Color) -> Option<Ray>;
}
