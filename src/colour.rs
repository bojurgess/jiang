use image::{ImageReader, Rgb};
use serde::{Deserialize, Serialize};
use std::{io::Cursor, ops::Index};
use tsify::Tsify;
use wasm_bindgen::JsError;

#[derive(Debug, Serialize, Deserialize, Tsify, Clone)]
#[serde(rename_all = "camelCase")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<&Rgb<u8>> for Colour {
    fn from(value: &Rgb<u8>) -> Self {
        Self::new(value[0], value[1], value[2])
    }
}

impl Index<usize> for Colour {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            _ => panic!("Tried to access invalid colour channel"),
        }
    }
}

impl Colour {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn to_hsl(&self) -> (f32, f32, f32) {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        let l = (max + min) / 2.0;

        let s = if delta == 0.0 {
            0.0
        } else {
            delta / (1.0 - (2.0 * l - 1.0).abs())
        };

        let h = if delta == 0.0 {
            0.0
        } else if max == r {
            60.0 * (((g - b) / delta) % 6.0)
        } else if max == g {
            60.0 * ((b - r) / delta + 2.0)
        } else {
            60.0 * ((r - g) / delta + 4.0)
        };

        ((h + 360.0) % 360.0, s, l)
    }
}

pub fn decode(data: &[u8]) -> Result<Vec<Colour>, image::ImageError> {
    let img = ImageReader::new(Cursor::new(data))
        .with_guessed_format()?
        .decode()?;

    let rgb: Vec<Colour> = img
        .into_rgb8()
        .pixels()
        .map(|p| Colour::new(p[0], p[1], p[2]))
        .collect();

    Ok(rgb)
}

pub fn from_rgba(data: &[u8]) -> Result<Vec<Colour>, JsError> {
    if data.len() % 4 != 0 {
        return Err(JsError::new("RGBA data length must be a multiple of 4"));
    }

    Ok(data
        .chunks_exact(4)
        .filter(|px| px[3] > 128)
        .map(|px| Colour::new(px[0], px[1], px[2]))
        .collect())
}
