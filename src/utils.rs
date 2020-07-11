use rand::Rng;
use std::f32::consts::PI;
use ultraviolet::Mat4;
use ultraviolet::Vec3;

#[allow(dead_code)]
pub fn random_unit_vector() -> Vec3 {
    let mut rng = rand::thread_rng();
    let z: f32 = rng.gen_range(-1.0, 1.0);
    let r = (1.0 - z.powi(2)).sqrt();
    let theta = rng.gen_range(0.0, 2.0 * PI);

    Vec3::new(r * theta.cos(), r * theta.sin(), z)
}

#[allow(dead_code)]
pub fn transform_swaps_handedness(_mat: &Mat4) -> bool {
    //     mat.slice_range(1.., 1..).determinant() < 0.0
    false
}
