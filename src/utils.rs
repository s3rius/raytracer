use std::f32::consts::PI;

#[must_use] 
pub const fn degrees_to_radians(degree: f32) -> f32 {
    degree * PI / 180.0
}
