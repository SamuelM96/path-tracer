mod camera;
mod colour;
mod intersectable;
mod utils;

use crate::camera::Camera;
use crate::colour::Colour;
use crate::intersectable::Intersectable;
use crate::sphere::Sphere;
use rand::Rng;
use std::sync::Arc;
use ultraviolet::Vec3;

mod sphere;

fn main() {
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 480;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
    const BACKGROUND: Colour = Colour {
        r: 10.0,
        g: 10.0,
        b: 15.0,
    };
    const SAMPLES: u32 = 100;

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

    let (tx, rx) = std::sync::mpsc::channel();
    let threadpool = threadpool::ThreadPool::new(num_cpus::get());
    let camera = Arc::new(camera);
    let sphere = Arc::new(sphere);

    for y in 0..IMAGE_HEIGHT {
        let tx = tx.clone();
        let camera = camera.clone();
        let sphere = sphere.clone();

        threadpool.execute(move || {
            let mut rng = rand::thread_rng();
            for x in 0..IMAGE_WIDTH {
                let mut pixel_colour = Colour::default();

                for _ in 0..SAMPLES {
                    let u = (x as f32 + rng.gen::<f32>()) / (IMAGE_WIDTH - 1) as f32;
                    let v = 1.0 - (y as f32 + rng.gen::<f32>()) / (IMAGE_HEIGHT - 1) as f32;
                    let ray = camera.get_ray(u, v);

                    if let Some(rec) = sphere.intersect(&ray) {
                        let cosine = rec.normal.dot(ray.direction);
                        let r = 200.0 * cosine;
                        let g = 0.0;
                        let b = 0.0;

                        pixel_colour += Colour::new(r, g, b) / SAMPLES as f32;
                    } else {
                        pixel_colour += BACKGROUND / SAMPLES as f32;
                    }
                }

                tx.send((x, y, pixel_colour)).unwrap();
            }
        });
    }

    for i in 0..(IMAGE_WIDTH * IMAGE_HEIGHT) {
        let (x, y, colour) = rx.recv().unwrap();
        image.put_pixel(x, y, image::Rgb(colour.to_u8()));
        println!("{}", i);
    }

    image
        .save("output.png")
        .expect("Couldn't save `output.png`");
}
