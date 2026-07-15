use crate::rgb::Rgb;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Swatch {
    pub hex: String,
    pub rgb: [u8; 3],
    pub hsl: [f32; 3],
    pub population: u32,
    pub title_text_color: String,
    pub body_text_color: String,
}

impl Swatch {
    pub fn from_color(colour: &Rgb, population: u32) -> Self {
        let (h, s, l) = colour.to_hsl();
        let hex = format!("#{:02X}{:02X}{:02X}", colour.r, colour.g, colour.b);
        let title_text_color = wcag_text_color(colour, 4.5);
        let body_text_color = wcag_text_color(colour, 3.0);

        Self {
            hex,
            rgb: [colour.r, colour.g, colour.b],
            hsl: [h, s, l],
            population,
            title_text_color,
            body_text_color,
        }
    }
}

fn wcag_text_color(bg: &Rgb, min_ratio: f32) -> String {
    let bg_lum = bg.relative_luminance();
    let white_ratio = contrast_ratio(1.0, bg_lum);
    let black_ratio = contrast_ratio(0.0, bg_lum);

    if white_ratio >= min_ratio {
        "#ffffff".to_string()
    } else if black_ratio >= min_ratio {
        "#000000".to_string()
    } else if white_ratio >= black_ratio {
        "#ffffff".to_string()
    } else {
        "#000000".to_string()
    }
}

fn contrast_ratio(text_lum: f32, bg_lum: f32) -> f32 {
    let lighter = text_lum.max(bg_lum);
    let darker = text_lum.min(bg_lum);
    (lighter + 0.05) / (darker + 0.05)
}
