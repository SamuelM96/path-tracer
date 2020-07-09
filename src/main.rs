mod camera;
mod colour;
mod intersectable;
mod scene;
mod utils;

use crate::camera::Camera;
use crate::colour::Colour;
use crate::intersectable::Intersectable;
use crate::scene::Scene;
use crate::sphere::Sphere;
use rand::Rng;
use std::sync::Arc;
use ultraviolet::geometry::Ray;
use ultraviolet::Vec3;

mod sphere;

fn cast_ray(ray: &Ray, scene: &Scene, depth: u32) -> Colour {
    if depth == 0 {
        return Colour::default();
    }

    let pixel_colour;

    if let Some(rec) = scene.intersect(&ray) {
        let cosine = rec.normal.dot(-ray.direction);
        let r = cosine;
        let g = 0.0;
        let b = 0.0;

        pixel_colour = Colour::new(r, g, b);
    } else {
        // Ray hit the nothing, i.e. the background
        pixel_colour = Colour::default();
    }

    pixel_colour
}

fn scene_setup(aspect_ratio: f32) -> (Scene, Camera) {
    // Scene Setup
    let mut scene = Scene::default();
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, 1.0), 1.0);
    scene.add(Box::new(sphere));

    // Camera Setup
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
        aspect_ratio,
        aperture,
        focus_distance,
    );

    (scene, camera)
}

fn main() {
    // Defaults
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 480;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
    const SAMPLES: u32 = 100;
    const MAX_DEPTH: u32 = 100;

    // Output image
    let mut image = image::ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    // Setup scene and camera
    let (scene, camera) = scene_setup(ASPECT_RATIO);

    // Concurrency setup
    let (tx, rx) = crossbeam::channel::unbounded();
    let thread_pool = threadpool::ThreadPool::new(num_cpus::get());
    let camera_lock = Arc::new(crossbeam::sync::ShardedLock::new(camera));
    let scene_lock = Arc::new(crossbeam::sync::ShardedLock::new(scene));

    // Render scene
    for y in 0..IMAGE_HEIGHT {
        let tx = tx.clone();
        let camera_lock = camera_lock.clone();
        let scene_lock = scene_lock.clone();

        // Process rows concurrently
        thread_pool.execute(move || {
            let mut rng = rand::thread_rng();
            let camera = camera_lock.read().unwrap();
            let scene = scene_lock.read().unwrap();

            for x in 0..IMAGE_WIDTH {
                let mut pixel_colour = Colour::default();

                // Jittery rays around
                for _ in 0..SAMPLES {
                    let u = (x as f32 + rng.gen::<f32>()) / (IMAGE_WIDTH - 1) as f32;
                    let v = 1.0 - (y as f32 + rng.gen::<f32>()) / (IMAGE_HEIGHT - 1) as f32;
                    let ray = camera.get_ray(u, v);

                    pixel_colour += cast_ray(&ray, &scene, MAX_DEPTH) / SAMPLES as f32;
                }

                // Output pixel colour
                pixel_colour.gamma_correct_mut();
                tx.send((x, y, pixel_colour)).unwrap();
            }
        });
    }

    // Collect results
    let mut percent = 0;
    const PIXEL_COUNT: u32 = IMAGE_WIDTH * IMAGE_HEIGHT;
    const QUARTER_COUNT: u32 = PIXEL_COUNT / 4;
    for i in 0..PIXEL_COUNT {
        let (x, y, colour) = rx.recv().unwrap();
        image.put_pixel(x, y, image::Rgb(colour.to_u8()));

        // Update user on progress
        if i % QUARTER_COUNT == 0 {
            percent += 25;
            println!("{}% complete...", percent);
        }
    }

    image
        .save("output.png")
        .expect("Couldn't save `output.png`");

    println!("Complete.");
}
