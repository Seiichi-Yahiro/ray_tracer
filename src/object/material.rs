use crate::intersection::TextureCoords;
use image::{DynamicImage, GenericImageView};
use std::ops::{Add, Mul};

pub struct Material {
    pub color: Coloration,
    pub albedo: f64,
    pub surface: SurfaceType,
}

pub enum SurfaceType {
    Diffuse,
    Reflective { reflectivity: f64 },
}

#[derive(Clone, Copy)]
pub struct Color(pub [f64; 3]);

impl Color {
    pub fn to_u8(&self) -> [u8; 4] {
        let [r, g, b] = self.0;
        [(r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8, 255]
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

pub enum Coloration {
    Color(Color),
    Texture(DynamicImage),
}

impl Coloration {
    pub fn color_at(&self, texture_coords: &TextureCoords) -> Color {
        match self {
            Coloration::Color(color) => *color,
            Coloration::Texture(texture) => {
                let tex_x = Self::wrap(texture_coords.x, texture.width());
                let tex_y = Self::wrap(texture_coords.y, texture.height());

                let [r, g, b, _] = texture.get_pixel(tex_x, tex_y).0;
                Color([r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0])
            }
        }
    }

    fn wrap(val: f64, bound: u32) -> u32 {
        let signed_bound = bound as i32;
        let float_coord = val * bound as f64;
        let wrapped_coord = (float_coord as i32) % signed_bound;

        if wrapped_coord < 0 {
            (wrapped_coord + signed_bound) as u32
        } else {
            wrapped_coord as u32
        }
    }
}
