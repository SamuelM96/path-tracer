use ultraviolet::geometry::Ray;
use ultraviolet::Vec3;

pub struct IntersectRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub distance: f32,
}

pub trait Intersectable: Send + Sync {
    fn intersect(&self, ray: &Ray) -> Option<IntersectRecord>;
}
