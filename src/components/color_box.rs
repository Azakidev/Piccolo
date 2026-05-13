/* MIT License
 *
 * Copyright (c) 2026 fatdawlf
 *
 * SPDX-License-Identifier: MIT
 */

use adw::{
    glib::{self, Properties},
    prelude::*,
    subclass::prelude::*,
};
use color::OpaqueColor;
use std::cell::{OnceCell, RefCell};

use crate::{components::color_functions::ColorFormat, window::WindowAction};

mod imp {

    use super::*;

    #[derive(Default, gtk::CompositeTemplate, Properties)]
    #[properties(wrapper_type = super::PiccoloColorBox)]
    #[template(resource = "/art/fatdawlf/Piccolo/color-box.ui")]
    pub struct PiccoloColorBox {
        // Children
        #[template_child]
        pub format: TemplateChild<gtk::Label>,
        #[template_child]
        pub function: TemplateChild<gtk::Label>,

        // Color
        pub color_format: OnceCell<ColorFormat>,

        #[property(get, set)]
        pub h: RefCell<f32>,
        #[property(get, set)]
        pub s: RefCell<f32>,
        #[property(get, set)]
        pub v: RefCell<f32>,
    }
    #[glib::object_subclass]
    impl ObjectSubclass for PiccoloColorBox {
        const NAME: &'static str = "PiccoloColorBox";
        type Type = super::PiccoloColorBox;
        type ParentType = adw::Bin;

        fn new() -> Self {
            Self {
                h: RefCell::new(0f32),
                s: RefCell::new(0f32),
                v: RefCell::new(0f32),
                ..Default::default()
            }
        }

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for PiccoloColorBox {
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

            obj.setup_title();
            obj.setup_listeners();
            obj.setup_click();
        }
    }
    impl WidgetImpl for PiccoloColorBox {}
    impl BinImpl for PiccoloColorBox {}
}

glib::wrapper! {
    pub struct PiccoloColorBox(ObjectSubclass<imp::PiccoloColorBox>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl PiccoloColorBox {
    pub fn new(format: ColorFormat) -> Self {
        let obj: Self = glib::Object::new();

        obj.imp().format.set_text(&format);

        if let Err(e) = obj.imp().color_format.set(format) {
            eprintln!("Error setting color format: {e}");
        }

        obj
    }

    fn setup_title(&self) {
        let imp = self.imp();
        if let Some(color_format) = imp.color_format.get() {
            imp.format.set_text(color_format);
        }
    }

    fn setup_listeners(&self) {
        self.connect_h_notify(move |b| {
            b.set_function();
        });
        self.connect_s_notify(move |b| {
            b.set_function();
        });
        self.connect_v_notify(move |b| {
            b.set_function();
        });
    }

    fn set_function(&self) {
        let imp = self.imp();
        let function = &imp.function;

        let hsv = OpaqueColor::new([self.h(), self.s(), self.v()]);

        if let Some(format) = imp.color_format.get() {
            let text = format.get_function(hsv);
            function.set_text(&text);
        }
    }

    fn setup_click(&self) {
        let controller = gtk::GestureClick::new();

        controller.connect_released(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            move |_, _, _, _| {
                let function = obj.imp().function.text().to_string();
                let _ = obj.activate_action(&WindowAction::ColorCopy, Some(&function.to_variant()));
            }
        ));

        self.add_controller(controller);
    }
}
