use crate::ray::Ray;
use crate::utils::gamma;
use std::ops::Mul;
use ultraviolet::{Mat4, Vec3, Vec4};

pub struct Bounds3 {
    pub p_min: Vec3,
    pub p_max: Vec3,
}

#[allow(dead_code)]
impl Bounds3 {
    pub fn new(p_min: Vec3, p_max: Vec3) -> Bounds3 {
        Bounds3 { p_min, p_max }
    }

    fn intersect_predicate(&self, ray: &Ray) -> Option<(f32, f32)> {
        let mut t0 = ray.t_min;
        let mut t1 = ray.t_max;

        for i in 0..3 {
            let inv_dir = 1.0 / ray.direction[i];
            let mut t_near = (self.p_min[i] - ray.origin[i]) * inv_dir;
            let mut t_far = (self.p_max[i] - ray.origin[i]) * inv_dir;
            if t_near > t_far {
                std::mem::swap(&mut t_near, &mut t_far);
            }
            t_far *= 1.0 + 2.0 * gamma(3);

            t0 = t_near.max(t0);
            t1 = t_far.max(t1);
            if t0 > t1 {
                return None;
            }
        }

        Some((t0, t1))
    }
}

impl Mul<Bounds3> for Mat4 {
    type Output = Bounds3;

    fn mul(self, rhs: Bounds3) -> Self::Output {
        let mut p_min = Vec4::new(rhs.p_min.x, rhs.p_min.y, rhs.p_min.z, 1.0);
        let mut p_max = Vec4::new(rhs.p_max.x, rhs.p_max.y, rhs.p_max.z, 1.0);

        p_min = self * p_min;
        p_max = self * p_max;

        Bounds3::new(p_min.xyz(), p_max.xyz())
    }
}
