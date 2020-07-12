mod bounds;
mod camera;
mod colour;
mod intersectable;
mod material;
mod ray;
mod scene;
mod shape;
mod utils;

use crate::camera::Camera;
use crate::colour::Colour;
use crate::intersectable::Intersectable;
use crate::material::{Diffuse, Light};
use crate::ray::Ray;
use crate::scene::Scene;
use crate::sphere::Sphere;
use rand::prelude::ThreadRng;
use rand::Rng;
use std::sync::Arc;
use ultraviolet::{Mat4, Vec3};

mod sphere;

#[allow(dead_code)]
fn debug_normals(
    ray: &Ray,
    scene: &Scene,
    depth: u32,
    stop_depth: u32,
    rng: &mut ThreadRng,
) -> Colour {
    if depth == 0 {
        return Colour::default();
    }

    if let Some((rec, _)) = scene.intersect(&ray, true) {
        if depth == stop_depth {
            return Colour::from(Vec3::new(0.5, 0.5, 0.5) + rec.normal * 0.5);
        }
        if let Some(material) = scene.materials.get(rec.material_id) {
            if let Some((scattered, _)) = material.scatter(ray, &rec, rng) {
                return debug_normals(&scattered, scene, depth - 1, stop_depth, rng);
            }
        } else {
            return Colour::error();
        }
    }

    Colour::default()
}

fn cast_ray(ray: &Ray, scene: &Scene, depth: u32, rng: &mut ThreadRng) -> Colour {
    if depth == 0 {
        return Colour::default();
    }

    let mut pixel_colour = Colour::default();

    // TODO: Implement Monte Carlo Integration and ray tracing
    if let Some((rec, _)) = scene.intersect(&ray, true) {
        if let Some(material) = scene.materials.get(rec.material_id) {
            let emitted = material.emitted(0.0, 0.0, &rec.point);
            if let Some((scattered, colour)) = material.scatter(ray, &rec, rng) {
                pixel_colour += emitted + colour * cast_ray(&scattered, scene, depth - 1, rng);
            } else {
                pixel_colour += emitted;
            }
        } else {
            pixel_colour = Colour::error();
        }
    }

    pixel_colour
}

fn scene_setup(aspect_ratio: f32) -> (Scene, Camera) {
    // Scene Setup
    let mut scene = Scene::default();

    // Materials Setup
    let ground_mat = scene.add_material(Box::new(Diffuse::new(Colour::new(0.7, 0.7, 0.7))));
    let left_wall_mat = scene.add_material(Box::new(Diffuse::new(Colour::new(0.6, 0.1, 0.1))));
    let right_wall_mat = scene.add_material(Box::new(Diffuse::new(Colour::new(0.1, 0.6, 0.1))));
    let sphere_mat = scene.add_material(Box::new(Diffuse::new(Colour::new(0.8, 0.8, 0.8))));
    // let sphere_mat2 = scene.add_material(Box::new(Diffuse::new(Colour::new(1.0, 0.0, 0.0))));
    let light_mat = scene.add_material(Box::new(Light::new(Colour::new(1.0, 1.0, 1.0), 20.0)));

    // Objects Setup
    let ground = Sphere::new(Vec3::new(0.0, -1001.0, 1.0), 1000.0, ground_mat, false);
    scene.add_object(Box::new(ground));
    let left_wall = Sphere::new(Vec3::new(-1003.0, 0.0, 1.0), 1000.0, left_wall_mat, false);
    scene.add_object(Box::new(left_wall));
    let right_wall = Sphere::new(Vec3::new(1003.0, 0.0, 1.0), 1000.0, right_wall_mat, false);
    scene.add_object(Box::new(right_wall));
    let back_wall = Sphere::new(Vec3::new(0.0, 0.0, 1003.0), 1000.0, ground_mat, false);
    scene.add_object(Box::new(back_wall));
    let ceiling = Sphere::new(Vec3::new(0.0, 1003.0, 1.0), 1000.0, ground_mat, false);
    scene.add_object(Box::new(ceiling));

    let sphere = Sphere::new(Vec3::new(1.0, 0.0, 1.0), 1.0, sphere_mat, false);
    scene.add_object(Box::new(sphere));
    let sphere2 = Sphere::new(Vec3::new(-1.0, 0.0, 1.0), 1.0, sphere_mat, false);
    scene.add_object(Box::new(sphere2));

    // Lighting setup
    let pos = Vec3::new(0.0, 3.0, 0.5);
    let otw = Mat4::from_translation(pos);
    let wto = Mat4::from_translation(-pos);
    let light = Sphere::from_transform(otw, wto, 0.5, light_mat, false);
    scene.add_light_pos(pos);
    scene.add_object(Box::new(light));

    // Camera Setup
    let origin = Vec3::new(0.0, 1.5, -5.0);
    let target = Vec3::new(0.0, 1.0, 0.0);
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
        0.001,
        std::f32::INFINITY,
    );

    (scene, camera)
}

fn main() {
    // TODO: Command line arguments
    // Defaults
    // const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const ASPECT_RATIO: f32 = 4.0 / 3.0;
    const IMAGE_WIDTH: u32 = 480;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
    const SAMPLES: u32 = 1000;
    const MAX_DEPTH: u32 = 10;
    const STOP_DEPTH: u32 = MAX_DEPTH;
    const DEBUG_NORMALS: bool = false;

    // Output image
    let mut image = image::ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    // TODO: Support parsing a file for scene setup
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

                    if DEBUG_NORMALS {
                        pixel_colour = debug_normals(&ray, &scene, MAX_DEPTH, STOP_DEPTH, &mut rng);
                    } else {
                        pixel_colour +=
                            cast_ray(&ray, &scene, MAX_DEPTH, &mut rng) / SAMPLES as f64;
                    }
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
            println!("\r{:>5.2}% complete...", percent);
        }
    }

    image
        .save("output.png")
        .expect("Couldn't save `output.png`");

    println!("Complete.");
}
