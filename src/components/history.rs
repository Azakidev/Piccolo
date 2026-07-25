/* MIT License
 *
 * Copyright (c) 2026 FatDawlf
 *
 * SPDX-License-Identifier: MIT
 */

use adw::{
    gio,
    glib::{self, Properties},
    prelude::*,
    subclass::prelude::*,
};
use color::OpaqueColor;
use std::cell::RefCell;

use crate::components::{history_chip::PiccoloHistoryChip, utils::Hsv};

mod imp {

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, Properties)]
    #[properties(wrapper_type = super::PiccoloHistory)]
    #[template(resource = "/art/fatdawlf/Piccolo/history.ui")]
    pub struct PiccoloHistory {
        #[template_child]
        pub stack: TemplateChild<adw::ViewStack>,
        #[template_child]
        pub list: TemplateChild<adw::WrapBox>,
        #[template_child]
        pub scroller: TemplateChild<gtk::EventControllerScroll>,

        pub recent_colors: RefCell<Vec<OpaqueColor<Hsv>>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PiccoloHistory {
        const NAME: &'static str = "PiccoloHistory";
        type Type = super::PiccoloHistory;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for PiccoloHistory {
        fn properties() -> &'static [glib::ParamSpec] {
            Self::derived_properties()
        }
        fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            Self::derived_set_property(self, id, value, pspec);
        }

        fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            Self::derived_property(self, id, pspec)
        }

        fn constructed(&self) {
            self.parent_constructed();

            self.obj().setup_scroll();
        }
    }

    impl WidgetImpl for PiccoloHistory {}
    impl BinImpl for PiccoloHistory {}
}

glib::wrapper! {
    pub struct PiccoloHistory(ObjectSubclass<imp::PiccoloHistory>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Native, gtk::Root, gtk::ShortcutManager, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gio::ActionGroup, gio::ActionMap;
}

impl PiccoloHistory {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn add_chip(&self, color: &OpaqueColor<Hsv>) {
        let chip = PiccoloHistoryChip::new(color);
        let mut recents = self.imp().recent_colors.borrow_mut();

        if !recents.contains(color) {
            recents.push(*color);
            self.imp().list.prepend(&chip);
            if self.imp().list.first_child().is_some() {
                self.imp().stack.set_visible_child_name("content");
            }
        }
    }

    pub fn remove_chip(&self, chip: &PiccoloHistoryChip) {
        let col = OpaqueColor::new([chip.h(), chip.s(), chip.v()]);
        let mut recents = self.imp().recent_colors.borrow_mut();

        if let Some(index) = recents.iter().position(|c| c == &col) {
            recents.remove(index);
            self.imp().list.remove(chip);
            if self.imp().list.first_child().is_none() {
                self.imp().stack.set_visible_child_name("empty");
            }
        }
    }

    fn setup_scroll(&self) {
        self.imp()
            .scroller
            .connect_scroll(move |controller, dx, dy| {
                // Get the parent scrolled window
                if let Some(widget) = controller.widget()
                    && let Some(sw) = widget.downcast_ref::<gtk::ScrolledWindow>()
                {
                    let hadj = sw.hadjustment();
                    let delta = if dy != 0.0 { dy } else { dx };
                    let new_val = hadj.value() + (delta * hadj.step_increment());
                    hadj.set_value(new_val.clamp(hadj.lower(), hadj.upper() - hadj.page_size()));
                }

                glib::Propagation::Stop
            });
    }
}
