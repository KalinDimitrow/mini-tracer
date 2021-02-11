use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use std::process::Output;

#[derive(Clone)]
pub struct Color {
    pub r : f32,
    pub g : f32,
    pub b : f32,
}

fn convert(value : f32) -> u8 {
    if value <= 0.0 {
        0
    } else if value >= 1.0 {
        255
    } else {
        (value * 255.0) as u8
    }
}

impl Color {
    pub fn new(r : f32, g : f32, b : f32) -> Self {
        Color {r, g, b}
    }
    pub fn to_srgb(&self) -> [u8;4] {

        [convert(self.r), convert(self.g), convert(self.b), 255]
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, rhs : Self) -> Self {
        Color {r : self.r + rhs.r, g : self.g + rhs.g, b : self.b + rhs.b}
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl Sub for Color {
    type Output = Self;
    fn sub(self, rhs : Self) -> Self::Output {
        Self::Output {r : self.r - rhs.r, g : self.g - rhs.g, b : self.b - rhs.b}
    }
}

impl SubAssign for Color {
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
    }
}

impl Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, rhs : Color) -> Self::Output {
        Color {r : rhs.r * self, g : rhs.g * self, b : rhs.b * self}
    }
}

impl Mul for Color {
    type Output = Color;
    fn mul(self, rhs : Color) -> Self::Output {
        Color {r : rhs.r * self.r, g : rhs.g * self.g, b : rhs.b * self.b}
    }
}

impl MulAssign<f32> for Color {
    fn mul_assign(&mut self, rhs : f32) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
    }
}

impl Div<Color> for f32 {
    type Output = Color;
    fn div(self, rhs : Color) -> Self::Output {
        let quotient = 1.0f32/self;
        Color {r : rhs.r * quotient, g : rhs.g * quotient, b : rhs.b * quotient}
    }
}

impl DivAssign<f32> for Color {
    fn div_assign(&mut self, rhs : f32) {
        let quotient = 1.0f32/rhs;
        self.r *= quotient;
        self.g *= quotient;
        self.b *= quotient;
    }
}