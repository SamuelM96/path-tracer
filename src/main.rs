mod camera;
mod colour;
mod intersectable;
mod material;
mod primitive;
mod scene;
mod utils;

use crate::camera::Camera;
use crate::colour::Colour;
use crate::intersectable::Intersectable;
use crate::material::{Diffuse, Light};
use crate::scene::Scene;
use crate::sphere::Sphere;
use rand::prelude::ThreadRng;
use rand::Rng;
use std::sync::Arc;
use ultraviolet::geometry::Ray;
use ultraviolet::Vec3;

mod sphere;

fn cast_ray(ray: &Ray, scene: &Scene, depth: u32, rng: &mut ThreadRng) -> Colour {
    if depth == 0 {
        return Colour::default();
    }

    let mut pixel_colour = Colour::default();

    // TODO: Implement Monte Carlo Integration and ray tracing
    if let Some(rec) = scene.intersect(&ray) {
        if let Some(material) = scene.materials.get(rec.material_id) {
            let emitted = material.emitted(0.0, 0.0, &rec.point);
            if let Some((scattered, colour)) = material.scatter(ray, &rec, rng) {
                pixel_colour += emitted + colour * cast_ray(&scattered, scene, depth - 1, rng);
            } else {
                pixel_colour += emitted;
            }
        } else {
            pixel_colour = Colour::new(1.0, 0.0, 1.0);
        }
    }

    pixel_colour
}

fn scene_setup(aspect_ratio: f32) -> (Scene, Camera) {
    // Scene Setup
    let mut scene = Scene::default();

    // Materials Setup
    let ground_mat = scene.add_material(Box::new(Diffuse::new(Colour::new(0.3, 0.3, 0.3))));
    let sphere_mat = scene.add_material(Box::new(Diffuse::new(Colour::new(1.0, 0.0, 0.0))));
    let light_mat = scene.add_material(Box::new(Light::new(Colour::new(10.0, 10.0, 10.0))));

    // Objects Setup
    let ground = Sphere::new(Vec3::new(0.0, -1001.0, 1.0), 1000.0, ground_mat);
    scene.add_object(Box::new(ground));
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, 1.0), 1.0, sphere_mat);
    scene.add_object(Box::new(sphere));

    // Lighting setup
    let light = Sphere::new(Vec3::new(2.0, 0.0, 0.5), 0.5, light_mat);
    scene.add_light_pos(light.centre.clone());
    scene.add_object(Box::new(light));

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
    const SAMPLES: u32 = 2000;
    const MAX_DEPTH: u32 = 10;

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

        // TODO: Separate image into tiles for each thread to work on
        // Process rows concurrently
        thread_pool.execute(move || {
            let camera = camera_lock.read().unwrap();
            let scene = scene_lock.read().unwrap();

            for x in 0..IMAGE_WIDTH {
                let mut pixel_colour = Colour::default();
                let mut rng = rand::thread_rng();

                // Jitter rays around
                for _ in 0..SAMPLES {
                    let u = (x as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                    let v = 1.0 - (y as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                    let ray = camera.get_ray(u, v, &mut rng);

                    pixel_colour += cast_ray(&ray, &scene, MAX_DEPTH, &mut rng) / SAMPLES as f64;
                }

                // Output pixel colour
                pixel_colour.gamma_correct_mut();
                tx.send((x, y, pixel_colour)).unwrap();
            }
        });
    }

    // Collect results
    const PIXEL_COUNT: u32 = IMAGE_WIDTH * IMAGE_HEIGHT;
    for i in 0..PIXEL_COUNT {
        let (x, y, colour) = rx.recv().unwrap();
        image.put_pixel(x, y, image::Rgb(colour.to_u8()));

        // TODO: Don't flood stdout
        // Update user on progress
        if i % IMAGE_WIDTH == 0 {
            let percent = i as f64 / PIXEL_COUNT as f64 * 100.0;
            println!("{:>5.2}% complete...", percent);
        }
    }

    image
        .save("output.png")
        .expect("Couldn't save `output.png`");

    println!("Complete.");
}
