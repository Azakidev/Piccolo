/* MIT License
 *
 * Copyright (c) 2026 FatDawlf
 *
 * SPDX-License-Identifier: MIT
 */

pub static APP_ID: &str = default_env(option_env!("APP_ID"), "art.fatdawlf.Piccolo");
pub static VERSION: &str = default_env(option_env!("VERSION"), "unknown");
pub static GETTEXT_PACKAGE: &str = default_env(option_env!("GETTEXT_PACKAGE"), "piccolo");
pub static LOCALEDIR: &str = default_env(option_env!("LOCALEDIR"), "/app/share/locale");
pub static PKGDATADIR: &str = default_env(option_env!("PKGDATADIR"), "/app/share/piccolo");

const fn default_env(v: Option<&'static str>, default: &'static str) -> &'static str {
    match v {
        Some(v) => v,
        None => default,
    }
}
