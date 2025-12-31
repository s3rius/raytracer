use crate::{
    interval::Interval,
    renderables::{HitRecord, RayData, Renderable},
};

type RenderableObject = dyn Renderable + Sync;

#[derive(Default, Debug)]
pub struct Scene {
    objects: Vec<Box<RenderableObject>>,
}

impl Scene {
    pub fn add_object(&mut self, object: Box<RenderableObject>) {
        self.objects.push(object);
    }

    pub fn add_obects(&mut self, objects: impl IntoIterator<Item = Box<RenderableObject>>) {
        self.objects.extend(objects);
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Box<RenderableObject>> {
        self.objects.get_mut(index)
    }
}

impl Renderable for Scene {
    fn hit(&self, ray: &RayData) -> Option<HitRecord> {
        let mut closest = ray.interval.max;
        let mut res = None;
        for obj in &self.objects {
            let tmp_res = obj.hit(&RayData {
                ray: ray.ray,
                interval: Interval::new(ray.interval.min, closest),
            });
            if let Some(hit) = tmp_res {
                res = Some(hit);
                closest = hit.distance;
            }
        }
        res
    }
}

impl Renderable for &Scene {
    fn hit(&self, ray: &RayData) -> Option<HitRecord> {
        (*self).hit(ray)
    }
}
