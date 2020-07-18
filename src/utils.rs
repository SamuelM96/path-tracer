use rand::prelude::ThreadRng;
use rand::Rng;
use std::f32::consts::PI;
use ultraviolet::Mat4;
use ultraviolet::Vec3;

#[allow(dead_code)]
pub fn random_unit_vector(rng: &mut ThreadRng) -> Vec3 {
    let z: f32 = rng.gen_range(-1.0, 1.0);
    let r = (1.0 - z.powi(2)).sqrt();
    let theta = rng.gen_range(0.0, 2.0 * PI);

    Vec3::new(r * theta.cos(), r * theta.sin(), z)
}

#[allow(dead_code)]
pub fn transform_swaps_handedness(m: &Mat4) -> bool {
    let det = m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
        - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
        + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0]);

    det < 0.0
}

#[allow(dead_code)]
pub fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vec3 {
    let s = rand_distr::UnitSphere;
    let v: [f32; 3] = rng.sample(s);

    Vec3::new(v[0], v[1], v[2])
}

#[allow(dead_code)]
#[inline]
pub fn gamma(n: i32) -> f32 {
    (n as f32 * std::f32::EPSILON) / (1.0 - n as f32 * std::f32::EPSILON)
}

#[allow(dead_code)]
#[inline]
pub fn quadratic(a: f64, b: f64, c: f64) -> Option<(f32, f32)> {
    let discriminator = b.powi(2) - 4.0 * a * c;

    if discriminator < 0.0 {
        None
    } else {
        let root = discriminator.sqrt();

        let q = if b < 0.0 {
            -0.5 * (b - root)
        } else {
            -0.5 * (b + root)
        };

        let mut t0 = (q / a) as f32;
        let mut t1 = (c / q) as f32;

        if t0 > t1 {
            std::mem::swap(&mut t0, &mut t1);
        }

        Some((t0, t1))
    }
}

// https://github.com/llogiq/partition/blob/master/src/lib.rs#L25
#[allow(dead_code)]
#[inline]
pub fn partition<T, P>(data: &mut [T], predicate: P) -> (&mut [T], &mut [T])
where
    P: Fn(&T) -> bool,
{
    let len = data.len();
    if len == 0 {
        return (&mut [], &mut []);
    }
    let (mut l, mut r) = (0, len - 1);
    loop {
        while l < len && predicate(&data[l]) {
            l += 1
        }
        while r > 0 && !predicate(&data[r]) {
            r -= 1
        }
        if l >= r {
            return data.split_at_mut(l);
        }
        data.swap(l, r);
    }
}

#[allow(dead_code)]
#[inline]
pub fn create_coordinates_system(n: &Vec3) -> (Vec3, Vec3) {
    let sign = 1.0_f32.copysign(n.z);
    let a = -1.0 / (sign + n.z);
    let b = n.x * n.y * a;
    let b1 = Vec3::new(1.0 + sign * n.x * n.x * a, sign * b, -sign * n.x);
    let b2 = Vec3::new(b, sign + n.y * n.y * a, -n.y);

    (b1, b2)
}

#[allow(dead_code)]
#[inline]
pub fn uniform_sample_hemisphere(r1: f32, r2: f32) -> Vec3 {
    let sin_theta = (1.0 - r1.powi(2)).sqrt();
    let phi = 2.0 * PI * r2;
    let x = sin_theta * phi.cos();
    let y = sin_theta * phi.sin();

    Vec3::new(x, y, r1)
}
