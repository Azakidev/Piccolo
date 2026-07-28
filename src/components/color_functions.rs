/* MIT License
 *
 * Copyright (c) 2026 FatDawlf
 *
 * SPDX-License-Identifier: MIT
 */

use std::ops::Deref;

use angular_units::Deg;
use prisma::{
    FromColor, Hsl, Hsv, Hwb, Lab, Lchab, Rgb,
    color_space::{ConvertToXyz, named::SRgb},
    encoding::EncodableColor,
    white_point::D65,
};

use crate::components::utils::Oklch;

#[derive(strum::Display, strum::AsRefStr, strum::EnumIter)]
pub enum ColorFormat {
    #[strum(to_string = "RGB")]
    Rgb,
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
    #[strum(to_string = "XYZ - D65")]
    Xyz,
}

impl Deref for ColorFormat {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl ColorFormat {
    pub fn get_function(&self, hsv: Hsv<f32, Deg<f32>>) -> String {
        let rgb = Rgb::from_color(&hsv);

        let srgb = rgb.srgb_encoded();
        let rgblin = rgb.linear();

        let rgba8: Rgb<u8> = rgblin.color_cast();

        let cs = SRgb::default();
        let xyz = cs.convert_to_xyz(&srgb);

        match self {
            Self::Rgb => {
                format!("rgb({}, {}, {})", rgba8.red(), rgba8.green(), rgba8.blue())
            }
            Self::Hsl => {
                let hsl: Hsl<f32, Deg<f32>> = Hsl::from_color(&rgb);
                format!(
                    "hsl({:.0}, {:.2}%, {:.2}%)",
                    hsl.hue().0.clamp(0., 360.),
                    (hsl.saturation() * 100.).clamp(0., 100.),
                    (hsl.lightness() * 100.).clamp(0., 100.)
                )
            }
            Self::Hsv => {
                format!(
                    "hsv({:.0}, {:.2}%, {:.2}%)",
                    hsv.hue().0.clamp(0., 360.),
                    (hsv.saturation() * 100.).clamp(0., 100.),
                    (hsv.value() * 100.).clamp(0., 100.)
                )
            }
            Self::Hwb => {
                let hwb: Hwb<f32, Deg<f32>> = Hwb::from_color(&rgb);
                format!(
                    "hwb({:.0}, {:.2}%, {:.2}%)",
                    hwb.hue().0.clamp(0., 360.),
                    (hwb.whiteness() * 100.).clamp(0., 100.),
                    (hwb.blackness() * 100.).clamp(0., 100.)
                )
            }
            Self::Oklab => {
                let oklab = oklab::srgb_f32_to_oklab(oklab::Rgb {
                    r: srgb.red(),
                    g: srgb.green(),
                    b: srgb.blue(),
                });
                format!(
                    "oklab({:.2}%, {:.2}, {:.2})",
                    (oklab.l * 100.).clamp(0., 100.),
                    oklab.a,
                    oklab.b
                )
            }
            Self::Oklch => {
                let oklab = oklab::srgb_f32_to_oklab(oklab::Rgb {
                    r: srgb.red(),
                    g: srgb.green(),
                    b: srgb.blue(),
                });
                let oklch = Oklch::from(oklab);
                format!(
                    "oklch({:.2}%, {:.2}, {:.0})",
                    oklch.l * 100.,
                    oklch.c,
                    oklch.h
                )
            }
            Self::Lab => {
                let lab: Lab<f32, D65> = Lab::from_xyz(&xyz, D65);
                format!(
                    "lab({:.2}%, {:.2}, {:.2})",
                    lab.L().clamp(0., 100.),
                    lab.a().clamp(0., 100.),
                    lab.b().clamp(0., 100.)
                )
            }
            Self::Lch => {
                let lab: Lab<f32, D65> = Lab::from_xyz(&xyz, D65);
                let lch: Lchab<f32, D65> = Lchab::from_color(&lab);
                format!(
                    "lch({:.2}%, {:.2}, {:.0})",
                    lch.L().clamp(0., 100.),
                    lch.chroma().clamp(0., 100.),
                    lch.hue().0.clamp(0., 360.)
                )
            }
            Self::Xyz => {
                format!(
                    "xyz({:.2}%, {:.2}, {:.2})",
                    (xyz.x() * 100.).clamp(0., 100.),
                    xyz.y().clamp(0., 1.),
                    xyz.z().clamp(0., 2.),
                )
            }
        }
    }
}
