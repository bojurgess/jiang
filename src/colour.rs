use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Colour {
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
