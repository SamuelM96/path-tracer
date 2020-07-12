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
