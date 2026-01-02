use rand::distr::uniform::SampleRange;

pub use glam::f32::Vec3;
pub type Point3 = Vec3;

pub trait Vec3Ext {
    fn rand_unit(rng: &mut impl rand::Rng) -> Self;
    fn rand_with_range(rng: &mut impl rand::Rng, range: impl SampleRange<f32> + Clone) -> Self;
    fn rand_on_hemisphere(rng: &mut impl rand::Rng, normal: Self) -> Self;
    fn near_zero(&self) -> bool;
    fn reflect(&self, normal: Self) -> Self;
}

impl Vec3Ext for Vec3 {
    fn rand_with_range(rng: &mut impl rand::Rng, range: impl SampleRange<f32> + Clone) -> Self {
        Self::new(
            rng.random_range(range.clone()),
            rng.random_range(range.clone()),
            rng.random_range(range),
        )
    }

    fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    fn reflect(&self, normal: Self) -> Self {
        self - 2. * self.dot(normal) * normal
    }

    fn rand_unit(rng: &mut impl rand::Rng) -> Self {
        loop {
            let vec = Self::rand_with_range(rng, -1.0..=1.0);
            let lens = vec.length_squared();
            if f32::EPSILON < lens && lens <= 1. {
                return vec.normalize();
            }
        }
    }

    fn rand_on_hemisphere(rng: &mut impl rand::Rng, normal: Self) -> Self {
        let unit = Self::rand_unit(rng);
        if unit.dot(normal) > 0. { unit } else { -unit }
    }
}
