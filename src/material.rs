use crate::colour::Colour;
use crate::intersectable::IntersectRecord;
use crate::ray::Ray;
use crate::utils::{create_coordinates_system, uniform_sample_hemisphere};
use rand::prelude::ThreadRng;
use rand::Rng;
use std::f32::consts::PI;
use ultraviolet::{Mat3, Vec3};

#[derive(Default, Copy, Clone, Debug)]
pub struct MaterialID(usize);

#[derive(Default)]
pub struct MaterialStore(Vec<Box<dyn Material>>);

#[allow(dead_code)]
impl MaterialStore {
    pub fn new() -> MaterialStore {
        MaterialStore(Vec::new())
    }

    pub fn add(&mut self, material: Box<dyn Material>) -> MaterialID {
        self.0.push(material);

        MaterialID(self.0.len() - 1)
    }

    pub fn get(&self, material_id: MaterialID) -> Option<&Box<dyn Material>> {
        self.0.get(material_id.0)
    }
}

// TODO: Add more materials
// TODO: Use true BSDFs
pub trait Material: Send + Sync {
    fn scatter(
        &self,
        _ray: &Ray,
        _rec: &IntersectRecord,
        _rng: &mut ThreadRng,
    ) -> Option<(Ray, Colour)> {
        None
    }

    fn emitted(&self, _u: f32, _v: f32, _point: &Vec3) -> Colour {
        Colour::default()
    }
}

pub struct Diffuse {
    pub albedo: Colour,
}

impl Diffuse {
    pub fn new(albedo: Colour) -> Diffuse {
        Diffuse { albedo }
    }
}

impl Material for Diffuse {
    fn scatter(
        &self,
        ray: &Ray,
        rec: &IntersectRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Ray, Colour)> {
        let r1 = rng.gen::<f32>();
        let r2 = rng.gen::<f32>();
        let n = rec.normal;
        let (nt, nb) = create_coordinates_system(&n);
        let scattered_local = uniform_sample_hemisphere(r1, r2);
        let scattered_dir = scattered_local.x * nt + scattered_local.y * nb + scattered_local.z * n;
        let scattered = Ray::new(rec.point, scattered_dir, ray.t_min, ray.t_max);
        let cosine = r1 / PI;
        let colour = self.albedo * cosine;

        Some((scattered, colour))
    }
}

pub struct Light {
    pub albedo: Colour,
    pub intensity: f32,
}

impl Light {
    pub fn new(albedo: Colour, intensity: f32) -> Light {
        Light { albedo, intensity }
    }
}

impl Material for Light {
    fn emitted(&self, _u: f32, _v: f32, _point: &Vec3) -> Colour {
        self.albedo * self.intensity
    }
}
