use ultraviolet::geometry::Ray;
use ultraviolet::Vec3;

pub struct IntersectRecord {
    pub point: Vec3,
    pub normal: Vec3,
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<IntersectRecord>;
}
