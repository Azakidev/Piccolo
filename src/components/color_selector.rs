/* MIT License
 *
 * Copyright (c) 2026 fatdawlf
 *
 * SPDX-License-Identifier: MIT
 */

use adw::{
    prelude::{RangeExt, WidgetExt},
    subclass::prelude::*,
};
use color::{AlphaColor, Hsl, OpaqueColor};
use gtk::{
    CssProvider, TemplateChild,
    gdk::Display,
    glib::{self, Properties, clone, object::ObjectExt},
    prelude::EditableExt,
};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use crate::components::{color_chip::PiccoloColorChip, utils::Hsv};

mod imp {

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, Properties)]
    #[properties(wrapper_type = super::PiccoloColorSelector)]
    #[template(resource = "/art/fatdawlf/Piccolo/color-selector.ui")]
    pub struct PiccoloColorSelector {
        #[template_child]
        pub chip: TemplateChild<PiccoloColorChip>,
        #[template_child]
        pub hex_label: TemplateChild<gtk::Entry>,
        // Sliders
        #[template_child]
        pub hue_slider: TemplateChild<gtk::Scale>,
        #[template_child]
        pub saturation_slider: TemplateChild<gtk::Scale>,
        #[template_child]
        pub value_slider: TemplateChild<gtk::Scale>,

        // Spin Buttons
        #[template_child]
        pub hue_label: TemplateChild<gtk::SpinButton>,
        #[template_child]
        pub saturation_label: TemplateChild<gtk::SpinButton>,
        #[template_child]
        pub value_label: TemplateChild<gtk::SpinButton>,

        // Adjustments
        #[template_child]
        pub hue_a: TemplateChild<gtk::Adjustment>,
        #[template_child]
        pub saturation_a: TemplateChild<gtk::Adjustment>,
        #[template_child]
        pub value_a: TemplateChild<gtk::Adjustment>,

        #[property(get, set)]
        pub h: RefCell<f32>,
        #[property(get, set)]
        pub s: RefCell<f32>,
        #[property(get, set)]
        pub v: RefCell<f32>,

        // Flags
        pub should_update: Rc<Cell<bool>>,
        pub css_provider: RefCell<CssProvider>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PiccoloColorSelector {
        const NAME: &'static str = "PiccoloColorSelector";
        type Type = super::PiccoloColorSelector;
        type ParentType = gtk::Box;

        fn new() -> Self {
            Self {
                h: RefCell::new(0f32),
                s: RefCell::new(0f32),
                v: RefCell::new(0f32),
                should_update: Rc::new(Cell::new(true)),
                css_provider: RefCell::new(CssProvider::new()),
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

    impl ObjectImpl for PiccoloColorSelector {
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

            obj.link_sliders();
            obj.link_hex_label();
            obj.link_chip();

            obj.setup_css_provider();
            obj.update_properties();
            obj.set_hex();
        }
    }
    impl WidgetImpl for PiccoloColorSelector {}
    impl BoxImpl for PiccoloColorSelector {}
}

glib::wrapper! {
    pub struct PiccoloColorSelector(ObjectSubclass<imp::PiccoloColorSelector>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl PiccoloColorSelector {
    fn link_sliders(&self) {
        let imp = self.imp();

        let hs = &imp.hue_slider;
        let hl = &imp.hue_label;
        let ha = &imp.hue_a;

        hs.connect_value_changed(clone!(
            #[weak(rename_to = obj)]
            self,
            move |s| {
                let val = s.value();
                let imp = obj.imp();
                let label = &imp.hue_label;

                label.set_value(val);

                if obj.imp().should_update.get() {
                    let _ = obj.activate_action("editor.set-color", None);
                    obj.update_properties();
                    obj.set_hex();
                }
            }
        ));

        hl.connect_value_changed(clone!(
            #[weak(rename_to = obj)]
            self,
            move |l| {
                let imp = obj.imp();
                let slider = &imp.hue_slider;
                let val = l.value();

                slider.set_value(val);
            }
        ));

        self.bind_property("h", &ha.get(), "value")
            .bidirectional()
            .sync_create()
            .build();

        let ss = &imp.saturation_slider;
        let sl = &imp.saturation_label;
        let sa = &imp.saturation_a;

        ss.connect_value_changed(clone!(
            #[weak(rename_to = obj)]
            self,
            move |s| {
                let val = s.value();
                let imp = obj.imp();
                let label = &imp.saturation_label;

                label.set_value(val);

                if obj.imp().should_update.get() {
                    let _ = obj.activate_action("editor.set-color", None);
                    obj.update_properties();
                    obj.set_hex();
                }
            }
        ));

        sl.connect_value_changed(clone!(
            #[weak(rename_to = obj)]
            self,
            move |l| {
                let imp = obj.imp();
                let slider = &imp.saturation_slider;
                let val = l.value();

                slider.set_value(val);
            }
        ));

        self.bind_property("s", &sa.get(), "value")
            .bidirectional()
            .sync_create()
            .build();

        let vs = &imp.value_slider;
        let vl = &imp.value_label;
        let va = &imp.value_a;

        vs.connect_value_changed(clone!(
            #[weak(rename_to = obj)]
            self,
            move |s| {
                let val = s.value();
                let imp = obj.imp();
                let label = &imp.value_label;

                label.set_value(val);

                if obj.imp().should_update.get() {
                    let _ = obj.activate_action("editor.set-color", None);
                    obj.update_properties();
                    obj.set_hex();
                }
            }
        ));

        vl.connect_value_changed(clone!(
            #[weak(rename_to = obj)]
            self,
            move |l| {
                let imp = obj.imp();
                let slider = &imp.value_slider;
                let val = l.value();

                slider.set_value(val);
            }
        ));

        self.bind_property("v", &va.get(), "value")
            .bidirectional()
            .sync_create()
            .build();
    }

    fn link_hex_label(&self) {
        let hex_label = &self.imp().hex_label;

        hex_label.connect_changed(clone!(
            #[weak(rename_to = obj)]
            self,
            move |_e| {
                let imp = obj.imp();

                if imp.should_update.get() {
                    obj.parse_hex();
                }
            }
        ));
    }

    fn link_chip(&self) {
        let chip = self.imp().chip.get();

        self.bind_property("h", &chip, "h").sync_create().build();
        self.bind_property("s", &chip, "s").sync_create().build();
        self.bind_property("v", &chip, "v").sync_create().build();
    }

    fn set_hex(&self) {
        let imp = self.imp();
        let hex_label = &imp.hex_label;
        let color: OpaqueColor<Hsv> = OpaqueColor::new([self.h(), self.s(), self.v()]);

        let [r, g, b, _] = color.to_rgba8().to_u8_array();
        let text = format!("#{:02X}{:02X}{:02X}", r, g, b);

        imp.should_update.set(false);
        hex_label.set_text(&text);
        imp.should_update.set(true);
    }

    fn parse_hex(&self) {
        let imp = self.imp();
        let hex_label = &imp.hex_label;

        if hex_label.text().len() < 6 {
            return;
        }

        let text = format!(
            "#{}",
            hex_label
                .text()
                .strip_prefix("#")
                .unwrap_or(&hex_label.text())
        );

        if let Ok(color) = color::parse_color(&text) {
            let rgb: AlphaColor<color::LinearSrgb> = color
                .convert(color::ColorSpaceTag::LinearSrgb)
                .to_alpha_color();
            let hsv: OpaqueColor<Hsv> = rgb.convert().discard_alpha();

            let [h, s, v] = hsv.components;

            imp.should_update.set(false);
            self.set_h(h);
            self.set_s(s);
            self.set_v(v);
            let _ = self.activate_action("editor.set-color", None);
            self.update_properties();
            imp.should_update.set(true);
        }
    }

    fn setup_css_provider(&self) {
        let provider = self.imp().css_provider.borrow();

        gtk::style_context_add_provider_for_display(
            &Display::default().unwrap(),
            &provider.clone(),
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    fn update_properties(&self) {
        let provider = self.imp().css_provider.borrow();

        let hsv: OpaqueColor<Hsv> = OpaqueColor::new([self.h(), self.s(), self.v()]);

        let sat = make_saturation_gradient(hsv);
        let val = make_value_gradient(hsv);

        provider.load_from_string(&format!(
            ":root {{ --sat_gradient: {}; --val_gradient: {}; }}",
            sat, val
        ));
    }
}

fn make_saturation_gradient(hsv: OpaqueColor<Hsv>) -> String {
    let [h, _, hsv_v] = hsv.components;

    let full_hsv: OpaqueColor<Hsv> = OpaqueColor::new([h, 100f32, hsv_v]);
    let full_hsl: OpaqueColor<Hsl> = full_hsv.convert();

    let [_, _, l] = full_hsl.components;

    format!(
        "linear-gradient(to right, \
         hsl({h}, 0%, {hsv_v}%), \
         hsl({h}, 100%, {l}%))"
    )
}

fn make_value_gradient(hsv: OpaqueColor<Hsv>) -> String {
    let [h, hsv_s, _] = hsv.components;

    let full_hsv: OpaqueColor<Hsv> = OpaqueColor::new([h, hsv_s, 100f32]);
    let full_hsl: OpaqueColor<Hsl> = full_hsv.convert();

    let [_, s, l] = full_hsl.components;

    format!(
        "linear-gradient(to right, \
         #000, \
         hsl({h}, {s}%, {l}%))",
    )
}
