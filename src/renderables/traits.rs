pub trait Renderable: std::fmt::Debug {
    fn hit(&self, ray: &super::RayData) -> Option<super::HitRecord>;
}
