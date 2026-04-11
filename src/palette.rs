use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::colour::Colour;

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Algorithm {
    MedianCut,
    Octree,
}

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PaletteOptions {
    pub algorithm: Algorithm,
    pub k: u32,
}

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Palette {
    pub vibrant: Option<Colour>,
}

pub fn score(candidates: &[Colour]) -> Palette {
    todo!()
}
