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

    pub fn intersect(&self, ray: &Ray) -> f32 {
        let oc = ray.origin - self.centre;
        let a = ray.direction.dot(ray.direction);
        let b: f32 = 2.0 * oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius.powi(2);
        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            -1.0
        } else {
            (-b - discriminant.sqrt()) / (2.0 * a)
        }
    }
}
