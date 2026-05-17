/* MIT License
 *
 * Copyright (c) 2026 FatDawlf
 *
 * SPDX-License-Identifier: MIT
 */

mod application;
mod components;
mod config;
mod window;

use self::application::PiccoloApplication;
use self::window::PiccoloWindow;

use config::{GETTEXT_PACKAGE, LOCALEDIR, PKGDATADIR};
use gettextrs::{bind_textdomain_codeset, bindtextdomain, textdomain};
use gtk::prelude::*;
use gtk::{gio, glib};

fn main() -> glib::ExitCode {
    // Set up gettext translations
    bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8")
        .expect("Unable to set the text domain encoding");
    textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    // Load resources
    let resources = gio::Resource::load(PKGDATADIR.to_owned() + "/piccolo.gresource")
        .expect("Could not load resources");
    gio::resources_register(&resources);

    // Create app
    let app = PiccoloApplication::new("art.fatdawlf.Piccolo", &gio::ApplicationFlags::empty());

    app.run()
}
