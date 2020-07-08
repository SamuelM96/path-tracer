use rand::Rng;
use std::f32::consts::PI;
use ultraviolet::vec::Vec3;

#[allow(dead_code)]
pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), 0.0);

        if p.mag_sq() >= 1.0 {
            continue;
        }

        return p;
    }
}

#[allow(dead_code)]
pub fn random_unit_vector() -> Vec3 {
    let mut rng = rand::thread_rng();
    let z: f32 = rng.gen_range(-1.0, 1.0);
    let r = (1.0 - z.powi(2)).sqrt();
    let theta = rng.gen_range(0.0, 2.0 * PI);

    Vec3::new(r * theta.cos(), r * theta.sin(), z)
}
