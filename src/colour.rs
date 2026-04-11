use serde::{Deserialize, Serialize};
use std::ops::Index;
use tsify::Tsify;

#[derive(Debug, Serialize, Deserialize, Tsify, Clone)]
#[serde(rename_all = "camelCase")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
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
        todo!()
    }

    pub fn luminance(&self) -> f32 {
        todo!()
    }
}

pub fn decode(data: &[u8]) -> Result<Vec<Colour>, image::ImageError> {
    todo!()
}
