use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = extractPalette)]
pub fn extract_palette(data: &[u8]) -> Result<JsValue, JsError> {
    todo!()
}

#[wasm_bindgen(js_name = medianCut)]
pub fn median_cut(rgba: &[u8], k: u32) -> Result<JsValue, JsError> {
    todo!()
}

#[wasm_bindgen(js_name = octreeQuantize)]
pub fn octree_quantize(rgba: &[u8], k: u32) -> Result<JsValue, JsError> {
    todo!()
}
