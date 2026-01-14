use glam::{EulerRot, Quat, Vec3};

use crate::vec3::Point3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub position: Point3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}

impl Transform {
    #[must_use]
    pub fn new(position: Point3) -> Self {
        Self {
            position,
            ..Default::default()
        }
    }

    pub fn euler_angles(&self, rot: EulerRot) -> (f32, f32, f32) {
        self.rotation.to_euler(rot)
    }

    pub fn lerp(self, to: &Transform, progress: f32) -> Transform {
        Self {
            position: self.position.lerp(to.position, progress),
            rotation: self.rotation.lerp(to.rotation, progress),
            scale: self.scale.lerp(to.scale, progress),
        }
    }

    pub fn slerp(self, to: &Transform, progress: f32) -> Transform {
        Self {
            position: self.position.slerp(to.position, progress),
            rotation: self.rotation.slerp(to.rotation, progress),
            scale: self.scale.slerp(to.scale, progress),
        }
    }
}
