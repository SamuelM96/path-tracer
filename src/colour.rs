use num::clamp;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[allow(dead_code)]
#[derive(Default, Debug, Copy, Clone)]
pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[allow(dead_code)]
impl Colour {
    pub fn new(r: f32, g: f32, b: f32) -> Colour {
        Colour { r, g, b }
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

impl Mul<f32> for Colour {
    type Output = Colour;

    fn mul(self, rhs: f32) -> Self::Output {
        Colour::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl MulAssign<f32> for Colour {
    fn mul_assign(&mut self, rhs: f32) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
    }
}

impl Div<f32> for Colour {
    type Output = Colour;

    fn div(self, rhs: f32) -> Self::Output {
        Colour::new(self.r / rhs, self.g / rhs, self.b / rhs)
    }
}

impl DivAssign<f32> for Colour {
    fn div_assign(&mut self, rhs: f32) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
    }
}
