use crate::bvh::BVHNode;
use crate::intersectable::{IntersectRecord, Intersectable};
use crate::material::{Material, MaterialID, MaterialStore};
use crate::ray::Ray;
use crate::shape::Shape;
use ultraviolet::Vec3;

#[derive(Default)]
pub struct Scene {
    pub objects: Vec<Box<dyn Shape>>,
    pub bvh: Option<BVHNode>,
    pub light_positions: Vec<Vec3>,
    pub materials: MaterialStore,
}

impl Scene {
    pub fn add_object(&mut self, object: Box<dyn Shape>) {
        self.objects.push(object);
    }

    pub fn add_light_pos(&mut self, light_pos: Vec3) {
        self.light_positions.push(light_pos);
    }

    pub fn add_material(&mut self, material: Box<dyn Material>) -> MaterialID {
        self.materials.add(material)
    }

    pub fn generate_bvh(&mut self) {
        self.bvh = Some(BVHNode::construct(&mut self.objects));
    }
}

impl Intersectable for Scene {
    fn intersect(&self, ray: &Ray, test_alpha_texture: bool) -> Option<(IntersectRecord, f32)> {
        if let Some(bvh) = &self.bvh {
            bvh.intersect(&self.objects, ray, test_alpha_texture)
        } else {
            panic!("Forgotten to generate BVH structure for scene")
        }
    }
}
