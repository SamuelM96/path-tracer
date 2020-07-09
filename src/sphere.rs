use crate::intersectable::{IntersectRecord, Intersectable};
use ultraviolet::geometry::Ray;
use ultraviolet::Vec3;

pub struct Sphere {
    centre: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(centre: ultraviolet::Vec3, radius: f32) -> Sphere {
        Sphere { centre, radius }
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<IntersectRecord> {
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

            if numerator > 0.0 {
                distance = numerator / a;
            } else if numerator2 > 0.0 {
                distance = numerator2 / a;
            } else {
                return None;
            }

            let point = ray.at_distance(distance);
            let normal = (point - self.centre).normalized();

            Some(IntersectRecord { point, normal, distance })
        }
    }
}
