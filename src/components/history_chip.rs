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
use angular_units::Deg;
use prisma::{FromColor, Hsv, Rgb};
use std::cell::RefCell;

use crate::{
    components::{color_chip::PiccoloColorChip, history::PiccoloHistory},
    window::WindowAction,
};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, Properties)]
    #[properties(wrapper_type = super::PiccoloHistoryChip)]
    #[template(resource = "/art/fatdawlf/Piccolo/history-chip.ui")]
    pub struct PiccoloHistoryChip {
        // Children
        #[template_child]
        pub container: TemplateChild<gtk::Box>,
        #[template_child]
        pub chip: TemplateChild<PiccoloColorChip>,
        #[template_child]
        pub hex: TemplateChild<gtk::Label>,
        #[template_child]
        pub rgb: TemplateChild<gtk::Label>,
        #[template_child]
        pub copy_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub remove_button: TemplateChild<gtk::Button>,

        // Properties
        #[property(get, set)]
        pub h: RefCell<f32>,
        #[property(get, set)]
        pub s: RefCell<f32>,
        #[property(get, set)]
        pub v: RefCell<f32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PiccoloHistoryChip {
        const NAME: &'static str = "PiccoloHistoryChip";
        type Type = super::PiccoloHistoryChip;
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

    impl ObjectImpl for PiccoloHistoryChip {
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

            obj.bind_color();
            obj.setup_remove();
            obj.setup_copy();
            obj.setup_click();
        }
    }

    impl WidgetImpl for PiccoloHistoryChip {}
    impl BinImpl for PiccoloHistoryChip {}
}

glib::wrapper! {
    pub struct PiccoloHistoryChip(ObjectSubclass<imp::PiccoloHistoryChip>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Native, gtk::Root, gtk::ShortcutManager, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl PiccoloHistoryChip {
    pub fn new(color: &Hsv<f32, Deg<f32>>) -> Self {
        let obj: PiccoloHistoryChip = glib::Object::builder()
            .property("h", color.hue().0)
            .property("s", color.saturation())
            .property("v", color.value())
            .build();

        obj.set_labels(color);

        obj
    }

    fn bind_color(&self) {
        let chip = &self.imp().chip.get();

        self.bind_property("h", chip, "h").sync_create().build();
        self.bind_property("s", chip, "s").sync_create().build();
        self.bind_property("v", chip, "v").sync_create().build();
    }

    fn set_labels(&self, hsv: &Hsv<f32, Deg<f32>>) {
        let hex_label = &self.imp().hex;
        let rgb_label = &self.imp().rgb;

        let rgb = Rgb::from_color(hsv);
        let rgb: Rgb<u8> = rgb.color_cast();

        let hex = format!("#{:02X}{:02X}{:02X}", rgb.red(), rgb.green(), rgb.blue());
        let rgb = format!("RGB: {}, {}, {}", rgb.red(), rgb.green(), rgb.blue());

        hex_label.set_label(&hex);
        rgb_label.set_label(&rgb);
    }

    fn setup_click(&self) {
        let container = &self.imp().container;
        let controller = gtk::GestureClick::new();

        controller.set_propagation_phase(gtk::PropagationPhase::Bubble);
        controller.set_propagation_limit(gtk::PropagationLimit::SameNative);

        controller.connect_released(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            move |_, _, _, _| {
                obj.chip_selected();
            }
        ));

        container.add_controller(controller);
    }

    fn setup_remove(&self) {
        let remove_button = &self.imp().remove_button;

        remove_button.connect_clicked(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            move |_| {
                if let Some(parent) = obj.ancestor(PiccoloHistory::static_type())
                    && let Some(history) = parent.downcast_ref::<PiccoloHistory>()
                {
                    history.remove_chip(&obj);
                }
            }
        ));
    }

    fn setup_copy(&self) {
        let copy_button = &self.imp().copy_button;
        let hsv = Hsv::new(Deg(self.h() % 360.), self.s(), self.v());
        let rgb = Rgb::from_color(&hsv);
        let rgb: Rgb<u8> = rgb.color_cast();

        let hex = format!("#{:02X}{:02X}{:02X}", rgb.red(), rgb.green(), rgb.blue());

        copy_button.connect_clicked(move |btn| {
            let _ = btn.activate_action(&WindowAction::ColorCopy, Some(&hex.to_variant()));
        });
    }

    fn chip_selected(&self) {
        let _ = self.activate_action(
            &WindowAction::ColorSet,
            Some(&(self.h() as f64, self.s() as f64, self.v() as f64).to_variant()),
        );
    }
}
