use jiang::extract_palette_from_rgba;
use wasm_bindgen_test::wasm_bindgen_test;

fn solid_rgba(r: u8, g: u8, b: u8, count: usize) -> Vec<u8> {
    (0..count).flat_map(|_| [r, g, b, 255]).collect()
}

#[wasm_bindgen_test]
fn test_solid_red_has_dominant() {
    let rgba = solid_rgba(255, 0, 0, 100 * 100);
    let palette = extract_palette_from_rgba(&rgba, Some(5)).unwrap();
    assert!(palette.dominant.is_some());
}

#[wasm_bindgen_test]
fn test_solid_red_dominant_is_red() {
    let rgba = solid_rgba(255, 0, 0, 100 * 100);
    let palette = extract_palette_from_rgba(&rgba, Some(5)).unwrap();
    let dominant = palette.dominant.unwrap();
    // solid red input should produce a dominant swatch that is red
    assert_eq!(dominant.rgb[0], 255);
    assert_eq!(dominant.rgb[1], 0);
    assert_eq!(dominant.rgb[2], 0);
}

#[wasm_bindgen_test]
fn test_dominant_hex_format() {
    let rgba = solid_rgba(255, 0, 0, 100 * 100);
    let palette = extract_palette_from_rgba(&rgba, Some(5)).unwrap();
    let hex = palette.dominant.unwrap().hex;
    assert!(hex.starts_with('#'));
    assert_eq!(hex.len(), 7);
}

#[wasm_bindgen_test]
fn test_title_text_color_is_black_or_white() {
    let rgba = solid_rgba(255, 0, 0, 100 * 100);
    let palette = extract_palette_from_rgba(&rgba, Some(5)).unwrap();
    let color = palette.dominant.unwrap().title_text_color;
    assert!(color == "#000000" || color == "#ffffff");
}

#[wasm_bindgen_test]
fn test_empty_input_errors() {
    let result = extract_palette_from_rgba(&[], Some(5));
    assert!(result.is_err());
}
