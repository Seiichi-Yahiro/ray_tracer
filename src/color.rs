use std::iter::Sum;
use std::ops::{Add, Div, Mul};

#[derive(Clone, Copy)]
pub struct Color(pub [f64; 3]);

const GAMMA: f64 = 2.2;

impl Color {
    pub fn to_u8(&self) -> [u8; 4] {
        let [r, g, b] = self.clamp().0;
        [
            (r.powf(1.0 / GAMMA) * 255.0) as u8,
            (g.powf(1.0 / GAMMA) * 255.0) as u8,
            (b.powf(1.0 / GAMMA) * 255.0) as u8,
            255,
        ]
    }

    pub fn clamp(&self) -> Color {
        let [r, g, b] = self.0;
        Color([r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0)])
    }
}

impl From<[f64; 3]> for Color {
    fn from(color: [f64; 3]) -> Self {
        Color(color)
    }
}

impl From<[u8; 3]> for Color {
    fn from(color: [u8; 3]) -> Self {
        let [r, g, b] = color;
        Color::from([r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0])
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        let [r, g, b] = self.0;
        let [rr, gg, bb] = rhs.0;
        Color([r + rr, g + gg, b + bb])
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        let [r, g, b] = self.0;
        let [rr, gg, bb] = rhs.0;
        Color([r * rr, g * gg, b * bb])
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        let [r, g, b] = self.0;
        Color([r * rhs, g * rhs, b * rhs])
    }
}

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Self::Output {
        let [r, g, b] = self.0;
        Color([r / rhs, g / rhs, b / rhs])
    }
}

impl Sum for Color {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Color([0.0; 3]), |acc, color| acc + color)
    }
}
