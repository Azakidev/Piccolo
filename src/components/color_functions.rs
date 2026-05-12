/* MIT License
 *
 * Copyright (c) 2026 FatDawlf
 *
 * SPDX-License-Identifier: MIT
 */

use std::ops::Deref;

use color::{Hsl, Hwb, Lab, Lch, Oklab, Oklch, OpaqueColor};

use crate::components::utils::Hsv;

#[derive(strum::Display, strum::AsRefStr, strum::EnumIter)]
pub enum ColorFormat {
    #[strum(to_string = "Rgb")]
    Rgb,
    #[strum(to_string = "Rgba")]
    Rgba,
    #[strum(to_string = "HSL")]
    Hsl,
    #[strum(to_string = "HSV")]
    Hsv,
    #[strum(to_string = "HWB")]
    Hwb,
    #[strum(to_string = "OkLab")]
    Oklab,
    #[strum(to_string = "OkLch")]
    Oklch,
    #[strum(to_string = "CIELab")]
    Lab,
    #[strum(to_string = "CIELch")]
    Lch,
}

impl Deref for ColorFormat {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl ColorFormat {
    pub fn get_function(&self, hsv: OpaqueColor<Hsv>) -> String {
        match self {
            Self::Rgb => {
                let [r, g, b, _a] = hsv.to_rgba8().to_u8_array();
                return format!("rgb({r}, {g}, {b})");
            }
            Self::Rgba => {
                let [r, g, b, a] = hsv.to_rgba8().to_u8_array();
                return format!("rgba({r}, {g}, {b}, {a})", a = a as f32 / 255.);
            }
            Self::Hsl => {
                let hsl: OpaqueColor<Hsl> = hsv.convert();
                let [h, s, l] = hsl.components;
                return format!("hsl({:.0}, {:.2}%, {:.2}%)", h, s, l);
            }
            Self::Hsv => {
                let [h, s, v] = hsv.components;
                return format!("hsl({:.0}, {:.02}%, {:.02}%)", h, s, v);
            }
            Self::Hwb => {
                let hwb: OpaqueColor<Hwb> = hsv.convert();
                let hue = hsv.components[0];
                let [_h, w, b] = hwb.components;
                return format!("hwb({:.0}, {:.2}%, {:.2}%)", hue, w, b);
            }
            Self::Oklab => {
                let oklab: OpaqueColor<Oklab> = hsv.convert();
                let [l, a, b] = oklab.components;
                return format!("oklab({:.2}%, {:.2}, {:.2})", 100. * l, a, b);
            }
            Self::Oklch => {
                let oklch: OpaqueColor<Oklch> = hsv.convert();
                let hue = hsv.components[0];
                let [l, c, _h] = oklch.components;
                return format!("oklch({:.2}%, {:.2}, {:.0})", 100. * l, c, hue);
            }
            Self::Lab => {
                let oklab: OpaqueColor<Lab> = hsv.convert();
                let [l, a, b] = oklab.components;
                return format!("lab({:.2}%, {:.2}, {:.2})", l, a, b);
            }
            Self::Lch => {
                let lch: OpaqueColor<Lch> = hsv.convert();
                let hue = hsv.components[0];
                let [l, c, _h] = lch.components;
                return format!("lch({:.2}%, {:.2}, {:.0})", l, c, hue);
            }
        }
    }
}
