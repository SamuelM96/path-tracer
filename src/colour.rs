use num::clamp;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use ultraviolet::Vec3;

#[allow(dead_code)]
#[derive(Default, Debug, Copy, Clone)]
pub struct Colour {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

#[allow(dead_code)]
impl Colour {
    pub fn new(r: f64, g: f64, b: f64) -> Colour {
        Colour { r, g, b }
    }

    pub fn new_f32(r: f64, g: f64, b: f64) -> Colour {
        Colour {
            r: r as f64,
            g: g as f64,
            b: b as f64,
        }
    }

    // Magenta for errors
    pub fn error() -> Colour {
        Colour {
            r: 1.0,
            g: 0.0,
            b: 1.0,
        }
    }

    pub fn to_u8(&self) -> [u8; 3] {
        [
            (256. * clamp(self.r, 0.0, 0.999)) as u8,
            (256. * clamp(self.g, 0.0, 0.999)) as u8,
            (256. * clamp(self.b, 0.0, 0.999)) as u8,
        ]
    }

    pub fn gamma_correct(&self) -> Colour {
        Colour::new(self.r.sqrt(), self.g.sqrt(), self.b.sqrt())
    }

    pub fn gamma_correct_mut(&mut self) {
        self.r = self.r.sqrt();
        self.g = self.g.sqrt();
        self.b = self.b.sqrt();
    }
}

impl Into<Vec3> for Colour {
    fn into(self) -> Vec3 {
        Vec3::new(self.r as f32, self.g as f32, self.b as f32)
    }
}

impl Add<Colour> for Colour {
    type Output = Colour;

    fn add(self, rhs: Colour) -> Self::Output {
        Colour::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl AddAssign<Colour> for Colour {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl Sub<Colour> for Colour {
    type Output = Colour;

    fn sub(self, rhs: Self) -> Self::Output {
        Colour::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl SubAssign<Colour> for Colour {
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
    }
}

impl Mul<Colour> for Colour {
    type Output = Colour;

    fn mul(self, rhs: Colour) -> Self::Output {
        Colour::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl MulAssign<Colour> for Colour {
    fn mul_assign(&mut self, rhs: Colour) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
    }
}

impl Mul<f64> for Colour {
    type Output = Colour;

    fn mul(self, rhs: f64) -> Self::Output {
        Colour::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl MulAssign<f64> for Colour {
    fn mul_assign(&mut self, rhs: f64) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
    }
}

impl Mul<Vec3> for Colour {
    type Output = Colour;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Colour::new(
            self.r * rhs.x as f64,
            self.g * rhs.y as f64,
            self.b * rhs.z as f64,
        )
    }
}

impl MulAssign<Vec3> for Colour {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.r *= rhs.x as f64;
        self.g *= rhs.y as f64;
        self.b *= rhs.z as f64;
    }
}

impl Div<f32> for Colour {
    type Output = Colour;

    fn div(self, rhs: f32) -> Self::Output {
        let inv = 1.0 / (rhs as f64);
        self * inv
    }
}

impl DivAssign<f32> for Colour {
    fn div_assign(&mut self, rhs: f32) {
        let inv = 1.0 / (rhs as f64);
        *self *= inv;
    }
}

impl Div<f64> for Colour {
    type Output = Colour;

    fn div(self, rhs: f64) -> Self::Output {
        let inv = 1.0 / rhs;
        self * inv
    }
}

impl DivAssign<f64> for Colour {
    fn div_assign(&mut self, rhs: f64) {
        let inv = 1.0 / rhs;
        *self *= inv;
    }
}
