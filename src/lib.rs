mod colour;
mod palette;
mod quantize;
mod utils;

use wasm_bindgen::prelude::*;

use crate::{palette::PaletteOptions, utils::set_panic_hook};

#[wasm_bindgen(js_name = extractPalette)]
pub fn extract_palette(data: &[u8], opts: Option<PaletteOptions>) -> Result<JsValue, JsError> {
    set_panic_hook();

    todo!()
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
