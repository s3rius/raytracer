use crate::scene::renderable::{HitRecord, RayData, Renderable};

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
        self.objects.get_mut(index)
    }

    #[must_use] 
    pub fn hit(&self, ray: &RayData) -> Option<HitRecord> {
        for obj in &self.objects {
            let hit = obj.hit(ray);
            // If we hit any object, we return early.
            if hit.is_some() {
                return hit;
            }
        }
        None
    }
}
