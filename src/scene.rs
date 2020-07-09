use crate::intersectable::{IntersectRecord, Intersectable};
use ultraviolet::geometry::Ray;

#[derive(Default)]
pub struct Scene {
    pub objects: Vec<Box<dyn Intersectable>>,
}

impl Scene {
    pub fn add(&mut self, object: Box<dyn Intersectable>) {
        self.objects.push(object);
    }
}

impl Intersectable for Scene {
    fn intersect(&self, ray: &Ray) -> Option<IntersectRecord> {
        let mut closest_distance = std::f32::INFINITY;
        let mut rec = None;

        for object in self.objects.iter() {
            if let Some(temp_rec) = object.intersect(ray) {
                if temp_rec.distance < closest_distance {
                    closest_distance = temp_rec.distance;
                    rec = Some(temp_rec);
                }
            }
        }

        rec
    }
}
