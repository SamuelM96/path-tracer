use crate::intersectable::{IntersectRecord, Intersectable};
use crate::material::{Material, MaterialID, MaterialStore};
use crate::ray::Ray;
use crate::shape::Shape;
use ultraviolet::Vec3;

#[derive(Default)]
pub struct Scene {
    pub objects: Vec<Box<dyn Shape>>,
    pub lights: Vec<Vec3>,
    pub materials: MaterialStore,
}

impl Scene {
    pub fn add_object(&mut self, object: Box<dyn Shape>) {
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
    fn intersect(&self, ray: &Ray, test_alpha_texture: bool) -> Option<(IntersectRecord, f32)> {
        let mut closest_distance = ray.t_max;
        let mut rec = None;
        let mut ray = ray.clone();

        for object in self.objects.iter() {
            if let Some((temp_rec, distance)) = object.intersect(&ray, test_alpha_texture) {
                if distance < closest_distance {
                    closest_distance = distance;
                    rec = Some(temp_rec);
                    ray.t_max = closest_distance;
                }
            }
        }

        if let Some(rec) = rec {
            Some((rec, closest_distance))
        } else {
            None
        }
    }
}
