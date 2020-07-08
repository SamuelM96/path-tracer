mod camera;
mod utils;

use crate::camera::Camera;
use crate::sphere::Sphere;
use rand::Rng;
use ultraviolet::geometry::Ray;
use ultraviolet::Vec3;

mod sphere;

fn main() {
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 480;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;

    let mut image = image::ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let sphere = Sphere::new(Vec3::new(0.0, 0.0, 1.0), 1.0);

    let origin = Vec3::new(0.0, 0.0, -5.0);
    let target = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let fov = 50.0;
    let aperture = 0.0;
    let focus_distance = 10.0;
    let camera = Camera::new(
        origin,
        target,
        up,
        fov,
        ASPECT_RATIO,
        aperture,
        focus_distance,
    );

    let mut rng = rand::thread_rng();
    for y in 0..IMAGE_HEIGHT {
        for x in 0..IMAGE_WIDTH {
            let pixel_colour: [u8; 3];
            let u = (x as f32 + rng.gen::<f32>()) / (IMAGE_WIDTH - 1) as f32;
            let v = 1.0 - (y as f32 + rng.gen::<f32>()) / (IMAGE_HEIGHT - 1) as f32;
            let ray = camera.get_ray(u, v);

            let (result, _t) = sphere.intersect(&ray);
            if result {
                pixel_colour = [200, 0, 0];
            } else {
                pixel_colour = [0, 0, 0];
            }
            image.put_pixel(x, y, image::Rgb(pixel_colour));
        }
    }

    image
        .save("output.png")
        .expect("Couldn't save `output.png`");
}
