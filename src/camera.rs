use crate::utils::random_in_unit_disk;
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

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;

        Ray {
            origin: self.origin + offset,
            direction: (self.lower_left_corner + self.horizontal * s + self.vertical * t
                - self.origin
                - offset)
                .normalized(),
        }
    }
}
