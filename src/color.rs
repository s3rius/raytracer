// Color works with u8 instead of f32,
// so it's okay to loose sign.
#![allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]

use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const WHITE: Self = Self::new(255, 255, 255);
    pub const BLACK: Self = Self::new(0, 0, 0);
    pub const RED: Self = Self::new(255, 0, 0);
    pub const GREEN: Self = Self::new(0, 255, 0);
    pub const BLUE: Self = Self::new(0, 0, 255);

    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl From<Vec3> for Color {
    fn from(value: Vec3) -> Self {
        Self {
            r: (value.x * 255.999) as u8,
            g: (value.y * 255.999) as u8,
            b: (value.z * 255.999) as u8,
        }
    }
}

impl From<Color> for Vec3 {
    fn from(value: Color) -> Self {
        Self {
            x: f32::from(value.r) / 255.,
            y: f32::from(value.g) / 255.,
            z: f32::from(value.b) / 255.,
        }
    }
}
