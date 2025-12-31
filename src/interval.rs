#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub const EMPTY: Self = Self::new(f32::INFINITY, -f32::INFINITY);
    pub const UNIVERSE: Self = Self::new(-f32::INFINITY, f32::INFINITY);

    #[must_use]
    pub const fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    #[must_use]
    pub fn len(&self) -> f32 {
        self.max - self.min
    }

    #[must_use]
    #[inline]
    pub fn contains(&self, value: f32) -> bool {
        self.min <= value && value <= self.max
    }

    #[must_use]
    #[inline]
    pub fn surrounds(&self, value: f32) -> bool {
        self.min < value && value < self.max
    }
}
