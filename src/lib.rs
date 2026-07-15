mod decode;
mod lab;
mod palette;
mod quantize;
mod rgb;
mod swatch;
mod utils;
use crate::{
    decode::decode, lab::Lab, palette::Palette, quantize::Algorithm, utils::set_panic_hook,
};
use wasm_bindgen::prelude::*;

#[derive(Debug, Default)]
#[wasm_bindgen]
pub struct ExtractOptions {
    pub k: Option<u32>,
}

#[wasm_bindgen(js_name = extractPalette)]
pub fn extract_palette(data: &[u8], k: Option<u32>) -> Result<Palette, JsError> {
    set_panic_hook();
    let k: usize = k.unwrap_or(5) as usize;
    let pixels = rgb::from_rgba(&decode(data)?)?;
    let lab_pixels: Vec<Lab> = pixels.iter().map(|c| Lab::from_srgb(c.clone())).collect();
    let candidates = quantize::quantize(&lab_pixels, Algorithm::MedianCut, k);
    Ok(palette::score(&candidates))
}

#[wasm_bindgen(js_name = extractPaletteFromRgba)]
pub fn extract_palette_from_rgba(rgba: &[u8], k: Option<u32>) -> Result<Palette, JsError> {
    set_panic_hook();
    let k = k.unwrap_or(5) as usize;
    let pixels = rgb::from_rgba(rgba)?;
    let lab_pixels: Vec<Lab> = pixels.iter().map(|c| Lab::from_srgb(c.clone())).collect();
    let candidates = quantize::quantize(&lab_pixels, Algorithm::MedianCut, k);
    Ok(palette::score(&candidates))
}
