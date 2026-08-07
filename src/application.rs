/* MIT License
 *
 * Copyright (c) 2026 FatDawlf
 *
 * SPDX-License-Identifier: MIT
 */

use adw::{prelude::*, subclass::prelude::*};
use gettextrs::gettext;
use gtk::{gio, glib};
use std::cell::Cell;

use crate::{PiccoloWindow, config::VERSION};

mod imp {

    use super::*;

    #[derive(Debug, Default)]
    pub struct PiccoloApplication {
        pub launch_pick: Cell<bool>,
    }

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
            obj.add_main_option(
                "pick",
                glib::Char::from(b'p'),
                glib::OptionFlags::NONE,
                glib::OptionArg::None,
                &gettext("Pick a color upon launch"),
                None,
            );
        }
    }

    impl ApplicationImpl for PiccoloApplication {
        fn activate(&self) {
            let application = self.obj();
            // Get the current window or create one if necessary
            let window = application.active_window().unwrap_or_else(|| {
                let window = PiccoloWindow::new(&*application);
                window.upcast()
            });

            if self.launch_pick.get()
                && let Some(win) = window.downcast_ref::<PiccoloWindow>()
            {
                win.pick_color_and_present();
            } else {
                window.present();
            }
        }

        fn handle_local_options(
            &self,
            options: &glib::VariantDict,
        ) -> std::ops::ControlFlow<glib::ExitCode> {
            if options.lookup_value("pick", None).is_some() {
                self.launch_pick.set(true);
            }

            std::ops::ControlFlow::Continue(())
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
            .property("resource-base-path", "/art/fatdawlf/Piccolo")
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
            .application_icon("art.fatdawlf.Piccolo")
            .developer_name("FatDawlf")
            .version(VERSION)
            .developers(vec!["FatDawlf https://fatdawlf.art"])
            // Translators: Replace "translator-credits" with your name/username, and optionally an email or URL.
            .translator_credits(gettext("translator-credits"))
            .copyright("© 2026 FatDawlf")
            .license_type(gtk::License::MitX11)
            .build();

        about.present(Some(&window));
    }
}
