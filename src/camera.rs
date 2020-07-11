use rand::Rng;
use rand_distr::{Distribution, UnitDisc};
use ultraviolet::geometry::Ray;
use ultraviolet::vec::Vec3;

#[allow(dead_code)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
    unit_disc: UnitDisc,
}

#[allow(dead_code)]
impl Camera {
    pub fn new(
        origin: Vec3,
        target: Vec3,
        up: Vec3,
        fov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_distance: f32,
    ) -> Camera {
        let theta = fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w: Vec3 = (origin - target).normalized();
        let u: Vec3 = up.cross(w).normalized();
        let v: Vec3 = w.cross(u);

        let horizontal = u * (focus_distance * viewport_width);
        let vertical = v * (focus_distance * viewport_height);
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_distance;

        let lens_radius = aperture / 2.0;

        let unit_disc = UnitDisc {};

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
            unit_disc,
        }
    }

    pub fn get_ray<R: Rng + ?Sized>(&self, s: f64, t: f64, sampler: &mut R) -> Ray {
        let rand_unit_disc: [f32; 2] = self.unit_disc.sample(sampler);
        let rand_unit_disc = Vec3::new(rand_unit_disc[0], rand_unit_disc[1], 0.0);
        let rd = rand_unit_disc * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;

        Ray {
            origin: self.origin + offset,
            direction: (self.lower_left_corner
                + self.horizontal * s as f32
                + self.vertical * t as f32
                - self.origin
                - offset)
                .normalized(),
        }
    }
}
