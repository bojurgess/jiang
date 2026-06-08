use crate::color::Color;
use crate::swatch::Swatch;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Palette {
    pub dominant: Option<Swatch>,
    pub accent: Option<Swatch>,
    pub subtle: Option<Swatch>,
    pub dark: Option<Swatch>,
    pub light: Option<Swatch>,
}

struct RoleTarget {
    target_s: f32,
    target_l: f32,
    min_s: f32,
    min_l: f32,
}

impl RoleTarget {
    fn new(target_s: f32, target_l: f32, min_s: f32, min_l: f32) -> Self {
        Self {
            target_s,
            target_l,
            min_s,
            min_l,
        }
    }
}

pub fn score(candidates: &[(Color, u32)]) -> Palette {
    let dominant = RoleTarget::new(0.5, 0.5, 0.0, 0.2);
    let accent = RoleTarget::new(1.0, 0.5, 0.35, 0.3);
    let subtle = RoleTarget::new(0.2, 0.5, 0.0, 0.3);
    let dark = RoleTarget::new(0.4, 0.2, 0.1, 0.0);
    let light = RoleTarget::new(0.4, 0.8, 0.1, 0.6);

    Palette {
        dominant: best_for(candidates, &dominant),
        accent: best_for(candidates, &accent),
        subtle: best_for(candidates, &subtle),
        dark: best_for(candidates, &dark),
        light: best_for(candidates, &light),
    }
}

fn best_for(candidates: &[(Color, u32)], role: &RoleTarget) -> Option<Swatch> {
    candidates
        .iter()
        .filter(|(c, _)| {
            let (_, s, l) = c.to_hsl();
            s >= role.min_s && l >= role.min_l
        })
        .max_by(|(a, _), (b, _)| {
            score_candidate(a, role.target_s, role.target_l)
                .partial_cmp(&score_candidate(b, role.target_s, role.target_l))
                .unwrap()
        })
        .map(|(colour, population)| Swatch::from_color(colour, *population))
}

fn score_candidate(c: &Color, target_s: f32, target_l: f32) -> f32 {
    let (_, s, l) = c.to_hsl();
    let s_score = 1.0 - (s - target_s).abs();
    let l_score = 1.0 - (l - target_l).abs();
    s_score + l_score
}
