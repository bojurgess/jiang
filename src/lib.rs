mod colour;
mod palette;
mod quantize;
mod utils;

use wasm_bindgen::prelude::*;

use crate::{
    colour::Colour,
    palette::{Algorithm, Palette, PaletteOptions},
    quantize::{median_cut, octree},
    utils::set_panic_hook,
};

#[wasm_bindgen(js_name = extractPalette)]
pub fn extract_palette(data: &[u8], opts: Option<PaletteOptions>) -> Result<Palette, JsError> {
    set_panic_hook();

    let opts = opts.unwrap_or_default();
    let pixels = colour::decode(data)?;

    let candidates = quantize(&pixels, opts.algorithm, opts.k as usize);
    Ok(palette::score(&candidates, opts.scoring_options))
}

#[wasm_bindgen(js_name = medianCut)]
pub fn median_cut(rgba: &[u8], k: u32) -> Result<Vec<Colour>, JsError> {
    set_panic_hook();

    let pixels = colour::from_rgba(rgba)?;
    Ok(quantize(&pixels, Algorithm::MedianCut, k as usize))
}

#[wasm_bindgen(js_name = octreeQuantize)]
pub fn octree_quantize(rgba: &[u8], k: u32) -> Result<Vec<Colour>, JsError> {
    set_panic_hook();

    let pixels = colour::from_rgba(rgba)?;
    Ok(quantize(&pixels, Algorithm::Octree, k as usize))
}

fn quantize(pixels: &[Colour], algorithm: Algorithm, k: usize) -> Vec<Colour> {
    match algorithm {
        Algorithm::MedianCut => median_cut::quantize(pixels, k),
        Algorithm::Octree => octree::quantize(pixels, k),
    }
}
