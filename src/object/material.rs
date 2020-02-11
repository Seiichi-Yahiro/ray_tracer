use crate::intersection::TextureCoords;
use image::{DynamicImage, GenericImageView};

pub struct Material {
    pub color: Coloration,
    pub albedo: f64,
}

pub enum Coloration {
    Color([f64; 3]),
    Texture(DynamicImage),
}

impl Coloration {
    pub fn color_at(&self, texture_coords: &TextureCoords) -> [f64; 3] {
        match self {
            Coloration::Color(color) => color.clone(),
            Coloration::Texture(texture) => {
                let tex_x = Self::wrap(texture_coords.x, texture.width());
                let tex_y = Self::wrap(texture_coords.y, texture.height());

                let [r, g, b, _] = texture.get_pixel(tex_x, tex_y).0;
                [r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0]
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
