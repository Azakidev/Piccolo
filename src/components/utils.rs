/* MIT License
 *
 * Copyright (c) 2026 FatDawlf
 *
 * SPDX-License-Identifier: MIT
 */

use angular_units::Deg;
use prisma::{FromColor, Hsv, Rgb};

use oklab::Oklab;

#[derive(Debug, Clone, Copy)]
pub struct Oklch {
    pub l: f32, // Lightness (0.0 to 1.0)
    pub c: f32, // Chroma (0.0 to ~0.37+)
    pub h: f32, // Hue angle in degrees (0.0 to 360.0)
}

impl From<Oklab> for Oklch {
    fn from(lab: Oklab) -> Self {
        let c = lab.a.hypot(lab.b);
        let mut h = lab.b.atan2(lab.a).to_degrees();
        if h < 0.0 {
            h += 360.0;
        }

        Oklch { l: lab.l, c, h }
    }
}

impl From<Oklch> for Oklab {
    fn from(lch: Oklch) -> Self {
        let h_rad = lch.h.to_radians();
        Oklab {
            l: lch.l,
            a: lch.c * h_rad.cos(),
            b: lch.c * h_rad.sin(),
        }
    }
}

pub fn to_rgba(hsv: &Hsv<f32, Deg<f32>>) -> gtk::gdk::RGBA {
    let rgb = Rgb::from_color(hsv);
    let rgb: Rgb<u8> = rgb.color_cast();

    gtk::gdk::RGBA::new(
        rgb.red() as f32 / 255.0,
        rgb.green() as f32 / 255.0,
        rgb.blue() as f32 / 255.0,
        1.0,
    )
}

pub fn parse_hex_to_rgb(hex: &str) -> Result<Rgb<f32>, String> {
    let hex = hex.trim_start_matches('#');

    if hex.len() != 6 {
        return Err("Hex string must be 6 characters long".to_string());
    }

    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|e| e.to_string())?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|e| e.to_string())?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|e| e.to_string())?;

    Ok(Rgb::new(
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
    ))
}
