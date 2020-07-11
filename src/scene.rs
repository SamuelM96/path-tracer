use crate::intersectable::{IntersectRecord, Intersectable};
use crate::material::{Material, MaterialID, MaterialStore};
use ultraviolet::geometry::Ray;
use ultraviolet::Vec3;

#[derive(Default)]
pub struct Scene {
    pub objects: Vec<Box<dyn Intersectable>>,
    pub lights: Vec<Vec3>,
    pub materials: MaterialStore,
}

impl Scene {
    pub fn add_object(&mut self, object: Box<dyn Intersectable>) {
        self.objects.push(object);
    }

    pub fn add_light_pos(&mut self, light_pos: Vec3) {
        self.lights.push(light_pos);
    }

    pub fn add_material(&mut self, material: Box<dyn Material>) -> MaterialID {
        self.materials.add(material)
    }
}

impl Intersectable for Scene {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<IntersectRecord> {
        let mut closest_distance = t_max;
        let mut rec = None;

        for object in self.objects.iter() {
            if let Some(temp_rec) = object.intersect(ray, t_min, closest_distance) {
                if temp_rec.distance < closest_distance {
                    closest_distance = temp_rec.distance;
                    rec = Some(temp_rec);
                }
            }
        }

        rec
    }
}
