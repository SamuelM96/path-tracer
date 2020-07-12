use crate::material::MaterialID;
use crate::ray::Ray;
use ultraviolet::Vec3;

pub struct IntersectRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub material_id: MaterialID,
}

pub trait Intersectable: Send + Sync {
    fn intersect(&self, ray: &Ray, test_alpha_textures: bool) -> Option<(IntersectRecord, f32)>;

    fn intersect_predicate(&self, ray: &Ray, test_alpha_textures: bool) -> bool {
        self.intersect(ray, test_alpha_textures).is_some()
    }
}
