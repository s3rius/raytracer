use crate::{
    color::Color,
    scene::renderable::{RayData, Renderable},
    vec3::Vec3,
};

type RenderableObject = dyn Renderable + Send + Sync;

#[derive(Default, Debug)]
pub struct Scene {
    objects: Vec<Box<RenderableObject>>,
}

impl Scene {
    pub fn add_object(&mut self, object: Box<RenderableObject>) {
        self.objects.push(object);
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Box<RenderableObject>> {
        return self.objects.get_mut(index);
    }

    pub fn hit(&self, ray: &RayData) -> Option<Color> {
        for obj in &self.objects {
            if let Some(hit) = obj.hit(&ray) {
                return Some(Color::from((hit.normal + Vec3::ONE) / 2.));
            }
        }
        None
    }
}
