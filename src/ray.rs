use std::ops::Mul;
use ultraviolet::{Mat4, Vec3, Vec4};

#[derive(Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub t_min: f32,
    pub t_max: f32,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, t_min: f32, t_max: f32) -> Ray {
        Ray {
            origin,
            direction,
            t_min,
            t_max,
        }
    }

    pub fn at(&self, distance: f32) -> Vec3 {
        self.origin + self.direction * distance
    }
}

impl Mul<&Ray> for Mat4 {
    type Output = Ray;

    fn mul(self, rhs: &Ray) -> Self::Output {
        Ray::new(
            (self * Vec4::new(rhs.origin.x, rhs.origin.y, rhs.origin.z, 1.0)).xyz(),
            rhs.direction,
            rhs.t_min,
            rhs.t_max,
        )
    }
}
