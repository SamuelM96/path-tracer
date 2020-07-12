use crate::bounds::Bounds3;
use crate::intersectable::{IntersectRecord, Intersectable};
use ultraviolet::{Mat4, Vec2, Vec3};

#[allow(dead_code)]
pub trait Shape: Intersectable {
    fn object_bounds(&self) -> Bounds3;

    fn world_bounds(&self) -> Bounds3 {
        *self.object_to_world() * self.object_bounds()
    }

    fn object_to_world(&self) -> &Mat4;

    fn world_to_object(&self) -> &Mat4;

    fn reverse_orientation(&self) -> bool;

    fn transform_swaps_handedness(&self) -> bool;

    fn area(&self) -> f32;

    fn pdf_wi(&self, rec: &IntersectRecord, wi: &Vec3) -> f32;

    fn pdf(&self, _rec: &IntersectRecord) -> f32 {
        1.0 / self.area()
    }

    fn sample(&self, point: &Vec2) -> IntersectRecord;

    fn sample_record(&self, _rec: &IntersectRecord, u: &Vec2) -> IntersectRecord {
        self.sample(u)
    }
}
