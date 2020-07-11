use crate::colour::Colour;
use crate::intersectable::IntersectRecord;
use rand::prelude::ThreadRng;
use rand::Rng;
use std::f64::consts::PI;
use ultraviolet::geometry::Ray;
use ultraviolet::Vec3;

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
        let w = if rec.normal.dot(ray.direction) < 0.0 {
            rec.normal
        } else {
            -rec.normal
        };
        let phi = rng.gen::<f64>() * 2.0 * PI;
        let r2: f64 = rng.gen();
        let sin_theta = r2.sqrt();
        let cos_theta = (1.0 - r2).sqrt();
        let u = (if w.x.abs() > 0.1 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        })
        .cross(w)
        .normalized();
        let v = w.cross(u);
        let d = (u * (phi.cos() * sin_theta) as f32
            + v * (phi.sin() * sin_theta) as f32
            + w * cos_theta as f32)
            .normalized();

        let scattered = Ray {
            origin: rec.point,
            direction: d,
        };
        let colour = self.albedo;

        Some((scattered, colour))
    }
}

pub struct Light {
    pub albedo: Colour,
}

impl Light {
    pub fn new(albedo: Colour) -> Light {
        Light { albedo }
    }
}

impl Material for Light {
    fn emitted(&self, _u: f32, _v: f32, _point: &Vec3) -> Colour {
        self.albedo
    }
}
