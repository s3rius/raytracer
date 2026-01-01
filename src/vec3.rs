use rand::distr::{Distribution, StandardUniform, uniform::SampleRange};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type Point3 = Vec3;

impl Vec3 {
    pub const ZERO: Self = Self::new(0., 0., 0.);
    pub const ONE: Self = Self::new(1., 1., 1.);

    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[must_use]
    pub const fn with_x(&self, x: f32) -> Self {
        Self { x, ..*self }
    }

    #[must_use]
    pub const fn with_y(&self, y: f32) -> Self {
        Self { y, ..*self }
    }

    #[must_use]
    pub const fn with_z(&self, z: f32) -> Self {
        Self { z, ..*self }
    }

    #[inline]
    #[must_use]
    pub fn len_squared(&self) -> f32 {
        self.z
            .mul_add(self.z, self.y.mul_add(self.y, self.x.powi(2)))
    }

    #[inline]
    #[must_use]
    pub fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }

    #[inline]
    #[must_use]
    pub fn normalize(&self) -> Self {
        self / self.len()
    }

    #[inline]
    #[must_use]
    pub fn dot(&self, other: Self) -> f32 {
        self.y
            .mul_add(other.y, self.z.mul_add(other.z, self.x * other.x))
    }

    pub fn rand_with_range(rng: &mut impl rand::Rng, range: impl SampleRange<f32> + Clone) -> Self {
        Self::new(
            rng.random_range(range.clone()),
            rng.random_range(range.clone()),
            rng.random_range(range),
        )
    }

    pub fn rand(rng: &mut impl rand::Rng) -> Self {
        rng.random()
    }

    pub fn rand_unit(rng: &mut impl rand::Rng) -> Self {
        loop {
            let vec = Self::rand_with_range(rng, -1.0..=1.0);
            let lens = vec.len_squared();
            if f32::EPSILON < lens && lens <= 1. {
                return vec.normalize();
            }
        }
    }

    pub fn rand_on_hemisphere(rng: &mut impl rand::Rng, normal: Self) -> Self {
        let unit = Self::rand_unit(rng);
        if unit.dot(normal) > 0. { unit } else { -unit }
    }

    #[inline]
    #[must_use]
    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y.mul_add(other.z, -(self.z * other.y)),
            y: self.z.mul_add(other.x, -(self.x * other.z)),
            z: self.x.mul_add(other.y, -(self.y * other.x)),
        }
    }
}

impl From<f32> for Vec3 {
    fn from(value: f32) -> Self {
        Self {
            x: value,
            y: value,
            z: value,
        }
    }
}

macro_rules! impl_binary_op {
    ($trait:tt,$method:ident) => {
        impl_binary_op!($trait, $method, f32);
        impl_binary_op!($trait, $method, i32);
        impl_binary_op!($trait, $method, u32);
        impl_binary_op!($trait, $method, usize);

        impl $trait<Vec3> for Vec3 {
            type Output = Vec3;

            #[inline]
            fn $method(self, rhs: Vec3) -> Self::Output {
                Self {
                    x: $trait::$method(self.x, rhs.x),
                    y: $trait::$method(self.y, rhs.y),
                    z: $trait::$method(self.z, rhs.z),
                }
            }
        }

        impl<'a> $trait<&'a Vec3> for Vec3 {
            type Output = Vec3;

            #[inline]
            fn $method(self, rhs: &'a Vec3) -> Self::Output {
                Self {
                    x: $trait::$method(self.x, rhs.x),
                    y: $trait::$method(self.y, rhs.y),
                    z: $trait::$method(self.z, rhs.z),
                }
            }
        }

        impl<'a> $trait<Vec3> for &'a Vec3 {
            type Output = Vec3;

            #[inline]
            fn $method(self, rhs: Vec3) -> Self::Output {
                Vec3 {
                    x: $trait::$method(self.x, rhs.x),
                    y: $trait::$method(self.y, rhs.y),
                    z: $trait::$method(self.z, rhs.z),
                }
            }
        }

        impl<'a> $trait<&'a Vec3> for &'a Vec3 {
            type Output = Vec3;

            #[inline]
            fn $method(self, rhs: &'a Vec3) -> Self::Output {
                Vec3 {
                    x: $trait::$method(self.x, rhs.x),
                    y: $trait::$method(self.y, rhs.y),
                    z: $trait::$method(self.z, rhs.z),
                }
            }
        }
    };
    ($trait:tt,$method:ident,$type:ty) => {
        impl $trait<$type> for Vec3 {
            type Output = Vec3;

            #[inline]
            fn $method(self, rhs: $type) -> Self::Output {
                Self {
                    x: $trait::$method(self.x, rhs as f32),
                    y: $trait::$method(self.y, rhs as f32),
                    z: $trait::$method(self.z, rhs as f32),
                }
            }
        }

        impl<'a> $trait<$type> for &'a Vec3 {
            type Output = Vec3;

            #[inline]
            fn $method(self, rhs: $type) -> Self::Output {
                Vec3 {
                    x: $trait::$method(self.x, rhs as f32),
                    y: $trait::$method(self.y, rhs as f32),
                    z: $trait::$method(self.z, rhs as f32),
                }
            }
        }

        impl $trait<Vec3> for $type {
            type Output = Vec3;

            #[inline]
            fn $method(self, rhs: Vec3) -> Self::Output {
                Vec3 {
                    x: $trait::$method(self as f32, rhs.x),
                    y: $trait::$method(self as f32, rhs.y),
                    z: $trait::$method(self as f32, rhs.z),
                }
            }
        }

        impl<'a> $trait<&'a Vec3> for $type {
            type Output = Vec3;

            #[inline]
            fn $method(self, rhs: &'a Vec3) -> Self::Output {
                Vec3 {
                    x: $trait::$method(self as f32, rhs.x),
                    y: $trait::$method(self as f32, rhs.y),
                    z: $trait::$method(self as f32, rhs.z),
                }
            }
        }
    };
}

macro_rules! impl_assign_op {
    ($trait:tt,$method:ident) => {
        /// Scalar assign op.
        impl $trait<f32> for Vec3 {
            #[inline]
            fn $method(&mut self, rhs: f32) {
                $trait::$method(&mut self.x, rhs);
                $trait::$method(&mut self.y, rhs);
                $trait::$method(&mut self.z, rhs);
            }
        }

        /// Vector to vector assign op
        impl $trait<Vec3> for Vec3 {
            #[inline]
            fn $method(&mut self, rhs: Vec3) {
                $trait::$method(&mut self.x, rhs.x);
                $trait::$method(&mut self.y, rhs.y);
                $trait::$method(&mut self.z, rhs.z);
            }
        }
    };
}

impl_binary_op!(Mul, mul);
impl_binary_op!(Add, add);
impl_binary_op!(Sub, sub);
impl_binary_op!(Div, div);

impl_assign_op!(AddAssign, add_assign);
impl_assign_op!(MulAssign, mul_assign);
impl_assign_op!(SubAssign, sub_assign);
impl_assign_op!(DivAssign, div_assign);

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        (*self).neg()
    }
}

impl Distribution<Vec3> for StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        Vec3::new(
            rng.random_range(0.0..1.0),
            rng.random_range(0.0..1.0),
            rng.random_range(0.0..1.0),
        )
    }
}
