use rand::Rng;
use std::f32::consts::PI;
use ultraviolet::vec::Vec3;

pub fn random_in_unit_disk() -> Vec3 {
    // let mut rng = rand::thread_rng();
    // let r = rng.gen::<f32>().sqrt();
    // let theta = rng.gen_range(0.0, 2.0 * PI);
    //
    // Vec3::new(r * theta.cos(), r * theta.sin(), 1.0)
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), 0.0);

        if p.mag_sq() >= 1.0 {
            continue;
        }

        return p;
    }
}
