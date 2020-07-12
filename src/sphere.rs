use crate::bounds::Bounds3;
use crate::intersectable::{IntersectRecord, Intersectable};
use crate::material::MaterialID;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::utils::transform_swaps_handedness;
use ultraviolet::{Mat4, Vec2, Vec3, Vec4};

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
        let half_b: f32 = oc.dot(ray.direction);
        let c = oc.mag_sq() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant < 0.0 {
            None
        } else {
            let root = discriminant.sqrt();
            let numerator = -half_b - root;
            let numerator2 = -half_b + root;
            let distance;

            if numerator > ray.t_min && numerator < ray.t_max {
                distance = numerator / a;
            } else if numerator2 > ray.t_min && numerator2 < ray.t_max {
                distance = numerator2 / a;
            } else {
                return None;
            }

            let point = ray.at(distance);
            let normal = (point - self.centre).normalized();

            Some((
                IntersectRecord {
                    point,
                    normal,
                    material_id: self.material_id,
                },
                distance,
            ))
        }
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

    fn area(&self) -> f32 {
        unimplemented!()
    }

    fn pdf_wi(&self, rec: &IntersectRecord, wi: &Vec3) -> f32 {
        unimplemented!()
    }

    fn pdf(&self, _rec: &IntersectRecord) -> f32 {
        unimplemented!()
    }

    fn sample(&self, point: &Vec2) -> IntersectRecord {
        unimplemented!()
    }

    fn sample_record(&self, _rec: &IntersectRecord, u: &Vec2) -> IntersectRecord {
        unimplemented!()
    }
}
