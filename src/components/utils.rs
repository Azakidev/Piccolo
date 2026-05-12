/* MIT License
 *
 * Copyright (c) 2026 FatDawlf
 *
 * SPDX-License-Identifier: MIT
 */

use std::{any::TypeId, ops::Deref};

use color::{ColorSpace, ColorSpaceLayout, Hsl, Hwb, Lab, Lch, Oklab, Oklch, OpaqueColor, Rgba8, Srgb};

#[derive(Clone, Copy, Debug)]
pub struct Hsv;

impl ColorSpace for Hsv {
    const TAG: Option<color::ColorSpaceTag> = None;

    const LAYOUT: color::ColorSpaceLayout = ColorSpaceLayout::HueFirst;

    const WHITE_COMPONENTS: [f32; 3] = [0f32, 0f32, 100f32];

    fn to_linear_srgb(src: [f32; 3]) -> [f32; 3] {
        hsv_to_srgb(src)
    }

    fn from_linear_srgb(src: [f32; 3]) -> [f32; 3] {
        srgb_to_hsv(src)
    }

    fn scale_chroma([h, s, l]: [f32; 3], scale: f32) -> [f32; 3] {
        [h, s * scale, l]
    }

    fn convert<TargetCS: ColorSpace>(src: [f32; 3]) -> [f32; 3] {
        if TypeId::of::<Self>() == TypeId::of::<TargetCS>() {
            src
        } else if TypeId::of::<TargetCS>() == TypeId::of::<Srgb>() {
            hsv_to_srgb(src)
        } else if TypeId::of::<TargetCS>() == TypeId::of::<Hsl>() {
            hsv_to_hsl(src)
        } else {
            let lin_rgb = Self::to_linear_srgb(src);
            TargetCS::from_linear_srgb(lin_rgb)
        }
    }

    fn clip(src: [f32; 3]) -> [f32; 3] {
        let [h, s, v] = src;

        [h, s.max(0f32), v.clamp(0f32, 100f32)]
    }
}

fn hsv_to_srgb([h, s, v]: [f32; 3]) -> [f32; 3] {
    let s = (s * 0.01).clamp(0f32, 1f32);
    let v = (v * 0.01).clamp(0f32, 1f32);

    // Standardize hue to [0, 360)
    let h_prime = h.rem_euclid(360f32);
    let c = v * s;
    let x = c * (1f32 - ((h_prime / 60f32).rem_euclid(2f32) - 1f32).abs());
    let m = v - c;

    let (r_temp, g_temp, b_temp) = match h_prime {
        hp if hp < 60f32 => (c, x, 0f32),
        hp if hp < 120f32 => (x, c, 0f32),
        hp if hp < 180f32 => (0f32, c, x),
        hp if hp < 240f32 => (0f32, x, c),
        hp if hp < 300f32 => (x, 0f32, c),
        _ => (c, 0f32, x),
    };

    [r_temp + m, g_temp + m, b_temp + m]
}

fn srgb_to_hsv(src: [f32; 3]) -> [f32; 3] {
    let rgb: OpaqueColor<Srgb> = OpaqueColor::new(src);
    let hsl: OpaqueColor<Hsl> = rgb.convert();

    hsl_to_hsv(hsl.components)
}

fn hsv_to_hsl(hsv: [f32; 3]) -> [f32; 3] {
    let [h, s_v, v] = hsv;
    let s_v = s_v * 0.01;
    let v = v * 0.01;

    let l = v * (1.0 - s_v / 2.0);

    let s_l = if l == 0.0 || l == 1.0 {
        0.0
    } else {
        (v - l) / l.min(1.0 - l)
    };

    [h, s_l * 100f32, l * 100f32]
}

fn hsl_to_hsv([h, s_l, l]: [f32; 3]) -> [f32; 3] {
    let s_l = s_l * 0.01;
    let l = l * 0.01;

    let v = l + s_l * l.min(1.0 - l);

    let s_v = if v == 0.0 { 0.0 } else { 2.0 * (1.0 - l / v) };

    [h, s_v * 100f32, v * 100f32]
}

pub fn to_rgba(hsv: &OpaqueColor<Hsv>) -> gtk::gdk::RGBA {
    let srgb: Rgba8 = hsv.to_rgba8();

    gtk::gdk::RGBA::new(
        srgb.r as f32 / 255.0,
        srgb.g as f32 / 255.0,
        srgb.b as f32 / 255.0,
        1.0,
    )
}

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
