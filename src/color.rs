use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const WHITE: Self = Color::new(255, 255, 255);
    pub const BLACK: Self = Color::new(0, 0, 0);
    pub const RED: Self = Color::new(255, 0, 0);
    pub const GREEN: Self = Color::new(0, 255, 0);
    pub const BLUE: Self = Color::new(0, 0, 255);

    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        return Self { r, g, b };
    }
}

impl From<Vec3> for Color {
    fn from(value: Vec3) -> Self {
        Self {
            r: (value.x * 255.) as u8,
            g: (value.y * 255.) as u8,
            b: (value.z * 255.) as u8,
        }
    }
}

impl From<Color> for Vec3 {
    fn from(value: Color) -> Self {
        Self {
            x: value.r as f32 / 255.,
            y: value.g as f32 / 255.,
            z: value.b as f32 / 255.,
        }
    }
}
