/* MIT License
 *
 * Copyright (c) 2026 FatDawlf
 *
 * SPDX-License-Identifier: MIT
 */

use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use gtk::{gio, glib};

use crate::config::VERSION;
use crate::PiccoloWindow;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct PiccoloApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for PiccoloApplication {
        const NAME: &'static str = "PiccoloApplication";
        type Type = super::PiccoloApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for PiccoloApplication {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_gactions();
            obj.set_accels_for_action("app.quit", &["<control>q"]);
        }
    }

    impl ApplicationImpl for PiccoloApplication {
        // We connect to the activate callback to create a window when the application
        // has been launched. Additionally, this callback notifies us when the user
        // tries to launch a "second instance" of the application. When they try
        // to do that, we'll just present any existing window.
        fn activate(&self) {
            let application = self.obj();
            // Get the current window or create one if necessary
            let window = application.active_window().unwrap_or_else(|| {
                let window = PiccoloWindow::new(&*application);
                window.upcast()
            });

            // Ask the window manager/compositor to present the window
            window.present();
        }
    }

    impl GtkApplicationImpl for PiccoloApplication {}
    impl AdwApplicationImpl for PiccoloApplication {}
}

glib::wrapper! {
    pub struct PiccoloApplication(ObjectSubclass<imp::PiccoloApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl PiccoloApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .property("resource-base-path", "/art/FatDawlf/Piccolo")
            .build()
    }

    fn setup_gactions(&self) {
        let quit_action = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| app.quit())
            .build();
        let about_action = gio::ActionEntry::builder("about")
            .activate(move |app: &Self, _, _| app.show_about())
            .build();
        self.add_action_entries([quit_action, about_action]);
    }

    fn show_about(&self) {
        let window = self.active_window().unwrap();
        let about = adw::AboutDialog::builder()
            .application_name("Piccolo")
            .application_icon("art.FatDawlf.Piccolo")
            .developer_name("FatDawlf <https://fatdawlf.art>")
            .version(VERSION)
            .developers(vec!["FatDawlf <https://fatdawlf.art>"])
            // Translators: Replace "translator-credits" with your name/username, and optionally an email or URL.
            .translator_credits(&gettext("translator-credits"))
            .copyright("© 2026 FatDawlf")
            .license_type(gtk::License::MitX11)
            .build();

        about.present(Some(&window));
    }
}
