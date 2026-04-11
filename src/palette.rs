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
    pub scoring_options: Option<ScoringOptions>,
}

impl Default for PaletteOptions {
    fn default() -> Self {
        Self {
            algorithm: Algorithm::MedianCut,
            k: 5,
            scoring_options: Some(ScoringOptions::default()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct RoleOptions {
    pub target_s: f32,
    pub target_l: f32,
    pub min_s: f32,
    pub min_l: f32,
}

impl RoleOptions {
    fn new(target_s: f32, target_l: f32, min_s: f32, min_l: f32) -> Self {
        Self {
            target_s,
            target_l,
            min_s,
            min_l,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ScoringOptions {
    pub dominant: Option<RoleOptions>,
    pub accent: Option<RoleOptions>,
    pub subtle: Option<RoleOptions>,
    pub dark: Option<RoleOptions>,
    pub light: Option<RoleOptions>,
}

impl Default for ScoringOptions {
    fn default() -> Self {
        Self {
            dominant: Some(RoleOptions::new(0.5, 0.5, 0.0, 0.2)),
            accent: Some(RoleOptions::new(1.0, 0.5, 0.35, 0.3)),
            subtle: Some(RoleOptions::new(0.2, 0.5, 0.0, 0.3)),
            dark: Some(RoleOptions::new(0.4, 0.2, 0.1, 0.0)),
            light: Some(RoleOptions::new(0.4, 0.8, 0.1, 0.6)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Palette {
    pub dominant: Option<Colour>,
    pub accent: Option<Colour>,
    pub subtle: Option<Colour>,
    pub dark: Option<Colour>,
    pub light: Option<Colour>,
}

pub fn score(candidates: &[Colour], opts: Option<ScoringOptions>) -> Palette {
    let resolved_opts = opts.unwrap_or_default();

    let dominant = resolved_opts.dominant.unwrap();
    let accent = resolved_opts.accent.unwrap();
    let subtle = resolved_opts.subtle.unwrap();
    let dark = resolved_opts.dark.unwrap();
    let light = resolved_opts.light.unwrap();

    Palette {
        dominant: best_for(
            candidates,
            dominant.target_s,
            dominant.target_l,
            dominant.min_s,
            dominant.min_l,
        ),
        accent: best_for(
            candidates,
            accent.target_s,
            accent.target_l,
            accent.min_s,
            accent.min_l,
        ),
        subtle: best_for(
            candidates,
            subtle.target_s,
            subtle.target_l,
            subtle.min_s,
            subtle.min_l,
        ),
        dark: best_for(
            candidates,
            dark.target_s,
            dark.target_l,
            dark.min_s,
            dark.min_l,
        ),
        light: best_for(
            candidates,
            light.target_s,
            light.target_l,
            light.min_s,
            light.min_l,
        ),
    }
}

fn best_for(
    candidates: &[Colour],
    target_s: f32,
    target_l: f32,
    min_s: f32,
    min_l: f32,
) -> Option<Colour> {
    candidates
        .iter()
        .filter(|c| {
            let (_, s, l) = c.to_hsl();
            s >= min_s && l >= min_l
        })
        .max_by(|a, b| {
            score_candidate(a, target_s, target_l)
                .partial_cmp(&score_candidate(b, target_s, target_l))
                .unwrap()
        })
        .cloned()
}

fn score_candidate(c: &Colour, target_s: f32, target_l: f32) -> f32 {
    let (_, s, l) = c.to_hsl();
    let s_score = 1.0 - (s - target_s).abs();
    let l_score = 1.0 - (l - target_l).abs();
    s_score + l_score
}
