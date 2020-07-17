use crate::ray::Ray;
use std::ops::Mul;
use ultraviolet::{Mat4, Vec3, Vec4};

#[derive(Copy, Clone, Default, Debug)]
pub struct Bounds3 {
    pub p_min: Vec3,
    pub p_max: Vec3,
    pub centroid: Vec3,
}

#[allow(dead_code)]
impl Bounds3 {
    pub fn new(p_min: Vec3, p_max: Vec3) -> Bounds3 {
        let centroid = p_min + (p_max - p_min) / 2.0;
        Bounds3 {
            p_min,
            p_max,
            centroid,
        }
    }

    pub fn maximum_extent(&self) -> usize {
        let diagonal = self.p_max - self.p_min;
        if diagonal.x > diagonal.y && diagonal.x > diagonal.z {
            0
        } else if diagonal.y > diagonal.z {
            1
        } else {
            2
        }
    }

    pub fn union(&self, b: &Bounds3) -> Bounds3 {
        Bounds3::new(
            self.p_min.min_by_component(b.p_min),
            self.p_max.max_by_component(b.p_max),
        )
    }

    pub fn union_point(&self, b: Vec3) -> Bounds3 {
        Bounds3::new(
            self.p_min.min_by_component(b),
            self.p_max.max_by_component(b),
        )
    }

    pub fn intersect_bounds(&self, b: &Bounds3) -> bool {
        self.p_max.x > b.p_min.x
            && self.p_min.x < b.p_max.x
            && self.p_max.y > b.p_min.y
            && self.p_min.y < b.p_max.y
            && self.p_max.z > b.p_min.z
            && self.p_min.z < b.p_max.z
    }

    pub fn intersect(&self, ray: &Ray) -> bool {
        let mut t_min = ray.t_min;
        let mut t_max = ray.t_max;

        for i in 0..3 {
            let inv_dir = 1.0 / ray.direction[i];
            let mut t0 = (self.p_min[i] - ray.origin[i]) * inv_dir;
            let mut t1 = (self.p_max[i] - ray.origin[i]) * inv_dir;
            if inv_dir < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            t_min = t0.max(t_min);
            t_max = t1.min(t_max);
            if t_max <= t_min {
                return false;
            }
        }

        true
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
