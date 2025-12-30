use std::fmt::Debug;

use crate::{color::Color, ray::Ray};

pub trait Renderable: Debug {
    fn render(&self, ray: &Ray) -> Option<Color>;
}
