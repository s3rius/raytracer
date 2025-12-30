use crate::{color::Color, ray::Ray, scene::renderable::Renderable};

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

    pub fn get_color(&self, ray: &Ray) -> Option<Color> {
        for obj in &self.objects {
            if let Some(color) = obj.render(&ray) {
                return Some(color);
            }
        }
        None
    }
}
