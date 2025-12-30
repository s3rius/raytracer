use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type Point3 = Vec3;

impl Vec3 {
    pub const ZERO: Vec3 = Vec3::new(0., 0., 0.);
    pub const ONE: Vec3 = Vec3::new(1., 1., 1.);

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub const fn with_x(&self, x: f32) -> Self {
        return Self { x: x, ..*self };
    }

    pub const fn with_y(&self, y: f32) -> Self {
        return Self { y: y, ..*self };
    }

    pub const fn with_z(&self, z: f32) -> Self {
        return Self { z: z, ..*self };
    }

    #[inline]
    pub fn len_squared(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    #[inline]
    pub fn len(&self) -> f32 {
        return self.len_squared().sqrt();
    }

    pub fn normalize(&self) -> Vec3 {
        self / self.len()
    }

    pub fn dot(&self, other: &Self) -> f32 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    pub fn cross(&self, other: &Self) -> Vec3 {
        return Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        };
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
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
