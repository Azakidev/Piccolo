/* MIT License
 *
 * Copyright (c) 2026 FatDawlf
 *
 * SPDX-License-Identifier: MIT
 */

use adw::{
    glib::{self, Properties},
    prelude::*,
    subclass::prelude::*,
};
use color::OpaqueColor;
use std::cell::RefCell;

use crate::components::utils::Hsv;

mod imp {

    use gtk::{graphene, gsk::RoundedRect};

    use crate::components::utils::to_rgba;

    use super::*;

    #[derive(Default, Properties)]
    #[properties(wrapper_type = super::PiccoloColorChip)]
    pub struct PiccoloColorChip {
        #[property(get, set)]
        pub h: RefCell<f32>,
        #[property(get, set)]
        pub s: RefCell<f32>,
        #[property(get, set)]
        pub v: RefCell<f32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PiccoloColorChip {
        const NAME: &'static str = "PiccoloColorChip";
        type Type = super::PiccoloColorChip;
        type ParentType = adw::Bin;

        fn new() -> Self {
            Self {
                h: RefCell::new(0f32),
                s: RefCell::new(0f32),
                v: RefCell::new(0f32),
            }
        }
    }

    impl ObjectImpl for PiccoloColorChip {
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

            let obj = self.obj();

            obj.add_css_class("color_chip");
            obj.add_css_class("frame");

            obj.setup_listeners();
        }
    }

    impl WidgetImpl for PiccoloColorChip {
        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            let obj = self.obj();
            let hsv: OpaqueColor<Hsv> = OpaqueColor::new([obj.h(), obj.s(), obj.v()]);
            let color = to_rgba(&hsv);

            let width = self.obj().width() as f32;
            let height = self.obj().height() as f32;

            let rect = graphene::Rect::new(0.0, 0.0, width, height);

            let radius = 8f32;

            let radius = radius.min(width / 2.0).min(height / 2.0);
            let rounded_rect = RoundedRect::from_rect(rect, radius);

            snapshot.push_rounded_clip(&rounded_rect);
            snapshot.append_color(&color, &rect);
            snapshot.pop();
        }
    }
    impl BinImpl for PiccoloColorChip {}
}

glib::wrapper! {
    pub struct PiccoloColorChip(ObjectSubclass<imp::PiccoloColorChip>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl PiccoloColorChip {
    fn setup_listeners(&self) {
        self.connect_h_notify(move |b| {
            b.queue_draw();
        });
        self.connect_s_notify(move |b| {
            b.queue_draw();
        });
        self.connect_v_notify(move |b| {
            b.queue_draw();
        });
    }
}
