use crate::bounds::Bounds3;
use crate::intersectable::{IntersectRecord, Intersectable};
use crate::material::MaterialID;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::utils::{quadratic, transform_swaps_handedness};
use ultraviolet::{Mat4, Rotor3, Vec2, Vec3, Vec4};

pub struct Cylinder {
    radius: f32,
    z_min: f32,
    z_max: f32,
    material_id: MaterialID,

    object_to_world: Mat4,
    world_to_object: Mat4,
    reverse_orientation: bool,
    transform_swaps_handedness: bool,
}

#[allow(dead_code)]
impl Cylinder {
    pub fn new(
        centre: Vec3,
        radius: f32,
        length: f32,
        rotation: Rotor3,
        scale: f32,
        material_id: MaterialID,
        reverse_orientation: bool,
    ) -> Cylinder {
        let half_len = length / 2.0;
        let z_min = -half_len;
        let z_max = half_len;
        let rot = rotation.into_matrix();
        let rot = Mat4::new(
            Vec4::new(rot[0][0], rot[1][0], rot[2][0], 0.0),
            Vec4::new(rot[0][1], rot[1][1], rot[2][1], 0.0),
            Vec4::new(rot[0][2], rot[1][2], rot[2][2], 0.0),
            Vec4::new(0.0, 0.0, 0.0, 0.0),
        );

        let object_to_world = Mat4::from_translation(centre) * Mat4::from_scale(scale) * rot;

        let rot = rotation.reversed().into_matrix();
        let rot = Mat4::new(
            Vec4::new(rot[0][0], rot[1][0], rot[2][0], 0.0),
            Vec4::new(rot[0][1], rot[1][1], rot[2][1], 0.0),
            Vec4::new(rot[0][2], rot[1][2], rot[2][2], 0.0),
            Vec4::new(0.0, 0.0, 0.0, 0.0),
        );
        let world_to_object = rot * Mat4::from_scale(1.0 / scale) * Mat4::from_translation(-centre);

        let transform_swaps_handedness = transform_swaps_handedness(&object_to_world);

        Cylinder {
            radius,
            z_min,
            z_max,
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
        z_min: f32,
        z_max: f32,
        material_id: MaterialID,
        reverse_orientation: bool,
    ) -> Cylinder {
        let transform_swaps_handedness = transform_swaps_handedness(&object_to_world);

        Cylinder {
            radius,
            z_min,
            z_max,
            material_id,
            object_to_world,
            world_to_object,
            reverse_orientation,
            transform_swaps_handedness,
        }
    }
}

impl Intersectable for Cylinder {
    fn intersect(&self, ray: &Ray, _test_alpha_textures: bool) -> Option<(IntersectRecord, f32)> {
        let r = self.world_to_object * ray;
        let o = r.origin;
        let a = r.direction.x.powi(2) + r.direction.y.powi(2);
        let b = 2.0 * (r.direction.x * o.x + r.direction.y * o.y);
        let c = o.x.powi(2) + o.y.powi(2) - self.radius.powi(2);

        if let Some((t0, t1)) = quadratic(a as f64, b as f64, c as f64) {
            if t0 > r.t_max || t1 < r.t_min {
                return None;
            }

            let z0 = o.z + t0 * r.direction.z;
            let z1 = o.z + t1 * r.direction.z;
            let t_hit;
            if t0 > r.t_min && self.z_min < z0 && z0 < self.z_max {
                t_hit = t0;
            } else if t1 > r.t_min && self.z_min < z1 && z1 < self.z_max {
                t_hit = t1;
            } else {
                return None;
            }

            let point = ray.at(t_hit);
            let normal = -ray.direction;

            return Some((
                IntersectRecord {
                    point,
                    normal,
                    material_id: self.material_id,
                },
                t_hit,
            ));
        }

        None
    }
}

impl Shape for Cylinder {
    fn object_bounds(&self) -> Bounds3 {
        unimplemented!()
    }

    fn world_bounds(&self) -> Bounds3 {
        self.object_to_world * self.object_bounds()
    }

    fn object_to_world(&self) -> &Mat4 {
        &self.object_to_world
    }

    fn world_to_object(&self) -> &Mat4 {
        &self.world_to_object
    }

    fn reverse_orientation(&self) -> bool {
        self.reverse_orientation
    }

    fn transform_swaps_handedness(&self) -> bool {
        self.transform_swaps_handedness
    }

    fn area(&self) -> f32 {
        (self.z_max - self.z_min) * self.radius
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
