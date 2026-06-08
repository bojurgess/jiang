mod color;
mod decode;
mod palette;
mod quantize;
mod swatch;
mod utils;

use crate::{decode::decode, palette::Palette, quantize::Algorithm, utils::set_panic_hook};
use wasm_bindgen::prelude::*;

/// Options for extractPalette. All fields are optional.
#[derive(Debug, Default)]
#[wasm_bindgen]
pub struct ExtractOptions {
    pub k: Option<u32>,
}

#[wasm_bindgen(js_name = extractPalette)]
pub fn extract_palette(data: &[u8], k: Option<u32>) -> Result<Palette, JsError> {
    set_panic_hook();
    let k: usize = k.unwrap_or(5) as usize;
    let pixels = color::from_rgba(&decode(data)?)?;
    let candidates = quantize::quantize(&pixels, Algorithm::MedianCut, k);
    Ok(palette::score(&candidates))
}

#[wasm_bindgen(js_name = extractPaletteFromRgba)]
pub fn extract_palette_from_rgba(rgba: &[u8], k: Option<u32>) -> Result<Palette, JsError> {
    set_panic_hook();
    let k = k.unwrap_or(5) as usize;
    let pixels = color::from_rgba(rgba)?;
    let candidates = quantize::quantize(&pixels, Algorithm::MedianCut, k);
    Ok(palette::score(&candidates))
}
