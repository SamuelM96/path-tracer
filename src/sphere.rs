use crate::bounds::Bounds3;
use crate::intersectable::{IntersectRecord, Intersectable};
use crate::material::MaterialID;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::utils::{quadratic, transform_swaps_handedness};
use std::f32::consts::PI;
use ultraviolet::{Mat4, Vec2, Vec3};

pub struct Sphere {
    pub centre: Vec3,
    pub radius: f32,
    pub material_id: MaterialID,

    object_to_world: Mat4,
    world_to_object: Mat4,
    reverse_orientation: bool,
    transform_swaps_handedness: bool,
}

impl Sphere {
    pub fn new(
        centre: Vec3,
        radius: f32,
        material_id: MaterialID,
        reverse_orientation: bool,
    ) -> Sphere {
        let object_to_world = Mat4::from_translation(centre);
        let world_to_object = Mat4::from_translation(-centre);
        let transform_swaps_handedness = transform_swaps_handedness(&object_to_world);

        Sphere {
            centre,
            radius,
            material_id,
            object_to_world,
            world_to_object,
            reverse_orientation,
            transform_swaps_handedness,
        }
    }

    pub fn from_transform(
        object_to_world: Mat4,
        world_to_object: Mat4,
        radius: f32,
        material_id: MaterialID,
        reverse_orientation: bool,
    ) -> Sphere {
        let transform_swaps_handedness = transform_swaps_handedness(&object_to_world);
        let centre = object_to_world.cols[3].xyz();

        Sphere {
            centre,
            radius,
            material_id,
            object_to_world,
            world_to_object,
            reverse_orientation,
            transform_swaps_handedness,
        }
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray, _test_alpha_textures: bool) -> Option<(IntersectRecord, f32)> {
        let oc = ray.origin - self.centre;
        let a = ray.direction.mag_sq();
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.mag_sq() - self.radius.powi(2);
        if let Some((t0, t1)) = quadratic(a as f64, b as f64, c as f64) {
            return if t0 > ray.t_max || t1 <= ray.t_min {
                None
            } else {
                let mut t_hit = t0;
                if t_hit <= ray.t_min {
                    t_hit = t1;
                    if t1 > ray.t_max {
                        return None;
                    }
                }

                let point = ray.at(t_hit);
                let normal = (point - self.centre).normalized();

                Some((
                    IntersectRecord {
                        point,
                        normal,
                        material_id: self.material_id,
                    },
                    t_hit,
                ))
            };
        }
        None
    }
}

impl Shape for Sphere {
    fn object_bounds(&self) -> Bounds3 {
        Bounds3::new(
            Vec3::new(-self.radius, -self.radius, -self.radius),
            Vec3::new(self.radius, self.radius, self.radius),
        )
    }

    #[inline]
    fn object_to_world(&self) -> &Mat4 {
        &self.object_to_world
    }

    #[inline]
    fn world_to_object(&self) -> &Mat4 {
        &self.world_to_object
    }

    #[inline]
    fn reverse_orientation(&self) -> bool {
        self.reverse_orientation
    }

    #[inline]
    fn transform_swaps_handedness(&self) -> bool {
        self.transform_swaps_handedness
    }

    #[inline]
    fn area(&self) -> f32 {
        4.0 * PI * self.radius.powi(2)
    }

    fn pdf_wi(&self, _rec: &IntersectRecord, _wi: &Vec3) -> f32 {
        unimplemented!()
    }

    fn pdf(&self, _rec: &IntersectRecord) -> f32 {
        unimplemented!()
    }

    fn sample(&self, _point: &Vec2) -> IntersectRecord {
        unimplemented!()
    }

    fn sample_record(&self, _rec: &IntersectRecord, _u: &Vec2) -> IntersectRecord {
        unimplemented!()
    }
}
