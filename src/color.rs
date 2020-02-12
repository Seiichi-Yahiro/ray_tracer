use std::ops::{Add, Mul};

#[derive(Clone, Copy)]
pub struct Color(pub [f64; 3]);

const GAMMA: f64 = 1.2;

impl Color {
    pub fn to_u8(&self) -> [u8; 4] {
        let [r, g, b] = self.0;
        [
            (r.powf(1.0 / GAMMA) * 255.0) as u8,
            (g.powf(1.0 / GAMMA) * 255.0) as u8,
            (b.powf(1.0 / GAMMA) * 255.0) as u8,
            255,
        ]
    }
}

impl From<[f64; 3]> for Color {
    fn from(color: [f64; 3]) -> Self {
        Color(color)
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
