use rand::prelude::ThreadRng;
use rand::Rng;
use rayon::prelude::*;
use std::io::Write;
use ultraviolet::{Mat4, Rotor3, Vec3};

use crate::camera::Camera;
use crate::colour::Colour;
use crate::cylinder::Cylinder;
use crate::intersectable::Intersectable;
use crate::material::{Diffuse, Light};
use crate::ray::Ray;
use crate::scene::Scene;
use crate::sphere::Sphere;
use std::f32::consts::PI;

mod bounds;
mod bvh;
mod camera;
mod colour;
mod cylinder;
mod intersectable;
mod material;
mod ray;
mod scene;
mod shape;
mod sphere;
mod utils;

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
                let pdf = material.pdf();
                let cosine = scattered.direction.dot(rec.normal) / PI;
                pixel_colour +=
                    emitted + colour * cosine * cast_ray(&scattered, scene, depth - 1, rng) / pdf;
            } else {
                pixel_colour += emitted;
            }
        } else {
            pixel_colour = Colour::error();
        }
    }
    // else {
    //     pixel_colour = Colour::new(1.0, 1.0, 1.0);
    // }

    pixel_colour
}

fn furnace_test(aspect_ratio: f32) -> (Scene, Camera) {
    let mut scene = Scene::default();

    let sphere_mat = scene.add_material(Box::new(Diffuse::new(Colour::new(0.18, 0.18, 0.18))));

    let sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0, sphere_mat, false);
    scene.add_object(Box::new(sphere));

    scene.generate_bvh();

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
        0.001,
        std::f32::INFINITY,
    );

    (scene, camera)
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
    // let cylinder_mat = scene.add_material(Box::new(Diffuse::new(Colour::new(0.9, 0.9, 0.9))));
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
    // let sphere2 = Sphere::new(Vec3::new(-1.0, 0.0, 1.0), 1.0, sphere_mat, false);
    // scene.add_object(Box::new(sphere2));

    // let pos = Vec3::new(-1.0, 0.0, 0.0);
    // let y_rot = 0.0_f32.to_radians();
    // let mut otw = Mat4::from_translation(pos);
    // let rot = Mat4::from_rotation_y(y_rot);
    // otw = otw * rot;
    // let mut wto = Mat4::from_translation(-pos);
    // let rot = Mat4::from_rotation_y(-y_rot);
    // wto = rot * wto;
    // let cylinder = Cylinder::from_transform(otw, wto, 0.5, -0.5, 0.5, cylinder_mat, false);
    // let cylinder = Cylinder::new(
    //     pos,
    //     0.5,
    //     1.0,
    //     Rotor3::from_rotation_xz(y_rot),
    //     1.0,
    //     cylinder_mat,
    //     false,
    // );
    // scene.add_object(Box::new(cylinder));

    // Lighting setup
    let pos = Vec3::new(0.0, 3.0, 0.5);
    let otw = Mat4::from_translation(pos);
    let wto = Mat4::from_translation(-pos);
    let light = Sphere::from_transform(otw, wto, 0.5, light_mat, false);
    scene.add_light_pos(pos);
    scene.add_object(Box::new(light));
    scene.generate_bvh();

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

struct Tile {
    pub x: u32,
    pub y: u32,
    pub data: image::RgbImage,
}

impl Tile {
    pub fn new(x: u32, y: u32, data: image::RgbImage) -> Tile {
        Tile { x, y, data }
    }
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
    const TILE_SIZE_X: u32 = 16;
    const TILE_SIZE_Y: u32 = 16;
    const TILES_X: u32 = (IMAGE_WIDTH + TILE_SIZE_X - 1) / TILE_SIZE_X;
    const TILES_Y: u32 = (IMAGE_HEIGHT + TILE_SIZE_Y - 1) / TILE_SIZE_Y;
    const TOTAL_TILES: u32 = TILES_X * TILES_Y;

    let time_date: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
    let time_start = std::time::Instant::now();

    // TODO: Support parsing a file for scene setup
    // Setup scene and camera
    let (scene, camera) = scene_setup(ASPECT_RATIO);
    // let (scene, camera) = furnace_test(ASPECT_RATIO);

    // Output image
    let mut image = image::ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    // Render scene
    let tiles = crossbeam::queue::ArrayQueue::new((TILES_X * TILES_Y) as usize);
    (0..IMAGE_HEIGHT)
        .into_par_iter()
        .step_by(TILE_SIZE_Y as usize)
        .for_each(|y| {
            let mut rng = rand::thread_rng();
            for x in (0..IMAGE_WIDTH).step_by(TILE_SIZE_X as usize) {
                // Current tile to render
                let mut tile = Tile::new(
                    x as u32,
                    y as u32,
                    image::RgbImage::new(TILE_SIZE_X, TILE_SIZE_Y),
                );

                // Core render loop
                for (tx, ty, pixel) in tile.data.enumerate_pixels_mut() {
                    let mut pixel_colour = Colour::default();

                    // TODO: Unroll and use SIMD vectors
                    // Jitter rays around
                    for _ in 0..SAMPLES {
                        let u = ((x + tx) as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                        let v =
                            1.0 - ((y + ty) as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                        let ray = camera.get_ray(u, v, &mut rng);

                        if DEBUG_NORMALS {
                            pixel_colour =
                                debug_normals(&ray, &scene, MAX_DEPTH, STOP_DEPTH, &mut rng);
                        } else {
                            pixel_colour +=
                                cast_ray(&ray, &scene, MAX_DEPTH, &mut rng) / SAMPLES as f64;
                        }
                    }

                    // Output pixel colour
                    pixel_colour.gamma_correct_mut();
                    *pixel = image::Rgb(pixel_colour.to_u8());
                }

                tiles.push(tile).unwrap();

                // TODO: Don't flood output
                // Update user on progress
                println!("{:>6.2}", tiles.len() as f64 / TOTAL_TILES as f64 * 100.0)
            }
        });

    // Draw tiles to output image
    while !tiles.is_empty() {
        let tile = tiles.pop().unwrap();
        image::imageops::overlay(&mut image, &tile.data, tile.x, tile.y);
    }

    // Save
    image
        .save("output.png")
        .expect("Couldn't save `output.png`");

    println!("Complete.");

    // Report execution time
    let time_end = time_start.elapsed();
    let total_seconds = time_end.as_secs() as f64 + (time_end.subsec_nanos() as f64 / 1000000000.0);
    println!("\nTotal Seconds: {}", total_seconds);

    // Keep track of results overall
    let mut results = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("results.csv")
        .unwrap();

    writeln!(
        &mut results,
        "{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
        time_date.to_rfc3339(),
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        TILES_X,
        TILE_SIZE_X,
        TILES_Y,
        TILE_SIZE_Y,
        SAMPLES,
        MAX_DEPTH,
        DEBUG_NORMALS,
        total_seconds
    )
    .unwrap();
}
