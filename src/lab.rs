use crate::rgb::Rgb;
use std::ops::Index;

#[derive(Debug, Clone, Copy)]
pub struct Lab {
    pub l: f32,
    pub a: f32,
    pub b: f32,
}

impl Index<usize> for Lab {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.l,
            1 => &self.a,
            2 => &self.b,
            _ => panic!("Attempted to access invalid colour channel"),
        }
    }
}

impl Lab {
    pub fn new(l: f32, a: f32, b: f32) -> Self {
        Self { l, a, b }
    }

    pub fn from_srgb(rgb: Rgb) -> Self {
        fn to_linear(c: u8) -> f32 {
            let c = c as f32 / 255.0;
            if c < 0.04045 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        }

        let r = to_linear(rgb.r);
        let g = to_linear(rgb.g);
        let b = to_linear(rgb.b);

        let l = 0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b;
        let m = 0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b;
        let s = 0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b;

        let l_ = l.cbrt();
        let m_ = m.cbrt();
        let s_ = s.cbrt();

        Lab {
            l: 0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_,
            a: 1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_,
            b: 0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_,
        }
    }

    pub fn to_srgb(&self) -> Rgb {
        let l_ = self.l + 0.3963377774 * self.a + 0.2158037573 * self.b;
        let m_ = self.l - 0.1055613458 * self.a - 0.0638541728 * self.b;
        let s_ = self.l - 0.0894841775 * self.a - 1.2914855480 * self.b;

        let l = l_ * l_ * l_;
        let m = m_ * m_ * m_;
        let s = s_ * s_ * s_;

        let r = 4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s;
        let g = -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s;
        let b = -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s;

        fn to_srgb_byte(c: f32) -> u8 {
            let c = c.clamp(0.0, 1.0);
            let encoded = if c <= 0.0031308 {
                c * 12.92
            } else {
                1.055 * c.powf(1.0 / 2.4) - 0.055
            };
            (encoded.clamp(0.0, 1.0) * 255.0).round() as u8
        }

        Rgb::new(to_srgb_byte(r), to_srgb_byte(g), to_srgb_byte(b))
    }
}
