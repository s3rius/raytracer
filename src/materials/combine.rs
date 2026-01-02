use std::sync::Arc;

use glam::Vec3;

use crate::materials::{Material, MaterialRecord};

#[derive(Debug, Default)]
pub struct CombineMaterial {
    pub materials: Vec<Arc<dyn Material>>,
}

impl CombineMaterial {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(mut self, material: Arc<dyn Material>) -> Self {
        self.materials.push(material);
        self
    }
}

impl Material for CombineMaterial {
    fn scatter(
        &self,
        ray_in: &crate::ray::Ray,
        hit: &crate::renderables::HitRecord,
    ) -> Option<super::MaterialRecord> {
        let mut ray = ray_in.clone();
        let mut albedo = Vec3::ZERO;

        for material in &self.materials {
            if let Some(mat_hit) = material.scatter(&ray, hit) {
                albedo += mat_hit.attenuation;
                ray = mat_hit.ray;
            }
        }
        return Some(MaterialRecord::new(
            albedo / self.materials.len() as f32,
            ray,
        ));
    }
}
