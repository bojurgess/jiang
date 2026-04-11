mod colour;
mod palette;
mod quantize;
mod utils;

use wasm_bindgen::prelude::*;

use crate::{
    palette::{Algorithm, PaletteOptions},
    quantize::{median_cut, octree},
    utils::set_panic_hook,
};

#[wasm_bindgen(js_name = extractPalette)]
pub fn extract_palette(data: &[u8], opts: Option<PaletteOptions>) -> Result<JsValue, JsError> {
    set_panic_hook();

    let (alg, k): (Algorithm, usize) = match opts {
        None => (Algorithm::MedianCut, 5),
        Some(opts) => (opts.algorithm, opts.k as usize),
    };

    let pixels = colour::decode(data)?;

    let candidates = match alg {
        Algorithm::MedianCut => median_cut::quantize(&pixels, k),
        Algorithm::Octree => octree::quantize(&pixels, k),
    };

    let palette = palette::score(&candidates);
    Ok(serde_wasm_bindgen::to_value(&palette)?)
}

#[wasm_bindgen(js_name = medianCut)]
pub fn median_cut(rgba: &[u8], k: u32) -> Result<JsValue, JsError> {
    set_panic_hook();

    todo!()
}

#[wasm_bindgen(js_name = octreeQuantize)]
pub fn octree_quantize(rgba: &[u8], k: u32) -> Result<JsValue, JsError> {
    set_panic_hook();

    todo!()
}
