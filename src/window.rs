/* MIT License
 *
 * Copyright (c) 2026 Unknown
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 * SPDX-License-Identifier: MIT
 */

use adw::{
    gio,
    glib::{self, Properties, VariantTy},
    prelude::*,
    subclass::prelude::*,
};
use ashpd::desktop::Color;
use color::{OpaqueColor, Srgb};
use gtk::gdk;
use std::{
    cell::RefCell,
    ops::{Deref, Sub},
};
use strum::IntoEnumIterator;

use crate::{
    components::{
        color_box::PiccoloColorBox,
        color_selector::PiccoloColorSelector,
        color_wheel::PiccoloColorWheel,
        utils::{ColorFormat, Hsv},
    },
    config,
};

mod imp {

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, Properties)]
    #[properties(wrapper_type = super::PiccoloWindow)]
    #[template(resource = "/art/FatDawlf/Piccolo/window.ui")]
    pub struct PiccoloWindow {
        // Template widgets
        #[template_child]
        pub toast_overlay: TemplateChild<adw::ToastOverlay>,
        #[template_child]
        pub sidebar_toggle: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub left_split: TemplateChild<adw::OverlaySplitView>,
        #[template_child]
        pub list: TemplateChild<gtk::Box>,
        #[template_child]
        pub wheel: TemplateChild<PiccoloColorWheel>,
        #[template_child]
        pub selector: TemplateChild<PiccoloColorSelector>,

        // Properties
        #[property(get, set)]
        pub h: RefCell<f32>,
        #[property(get, set)]
        pub s: RefCell<f32>,
        #[property(get, set)]
        pub v: RefCell<f32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PiccoloWindow {
        const NAME: &'static str = "PiccoloWindow";
        type Type = super::PiccoloWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();

            WindowAction::init_actions(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for PiccoloWindow {
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

            if config::APP_ID.ends_with(".Devel") {
                obj.add_css_class("devel");
            }

            obj.bind_sidebar_toggle();
            obj.bind_wheel();
            obj.bind_selector();
            obj.setup_boxes();
        }
    }
    impl WidgetImpl for PiccoloWindow {}
    impl WindowImpl for PiccoloWindow {}
    impl ApplicationWindowImpl for PiccoloWindow {}
    impl AdwApplicationWindowImpl for PiccoloWindow {}
}

glib::wrapper! {
    pub struct PiccoloWindow(ObjectSubclass<imp::PiccoloWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gtk::Native, gtk::Root, gtk::ShortcutManager, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gio::ActionGroup, gio::ActionMap;
}

impl PiccoloWindow {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }

    fn clear_focus(&self) {
        adw::prelude::GtkWindowExt::set_focus(self, None::<&gtk::Widget>);
    }

    fn bind_sidebar_toggle(&self) {
        let imp = self.imp();
        let toggle = &imp.sidebar_toggle;
        let sidebar = &imp.left_split.get();

        toggle
            .bind_property("active", sidebar, "show-sidebar")
            .sync_create()
            .build();
    }

    fn bind_wheel(&self) {
        let imp = self.imp();
        let wheel = &imp.wheel.get();

        self.bind_property("h", wheel, "h")
            .bidirectional()
            .sync_create()
            .build();
        self.bind_property("s", wheel, "s")
            .bidirectional()
            .sync_create()
            .build();
        self.bind_property("v", wheel, "v")
            .bidirectional()
            .sync_create()
            .build();
    }

    fn bind_selector(&self) {
        let imp = self.imp();
        let selector = &imp.selector.get();

        self.bind_property("h", selector, "h")
            .bidirectional()
            .sync_create()
            .build();
        self.bind_property("s", selector, "s")
            .bidirectional()
            .sync_create()
            .build();
        self.bind_property("v", selector, "v")
            .bidirectional()
            .sync_create()
            .build();
    }

    fn set_color(&self, color: Color) {
        let col: OpaqueColor<Srgb> = OpaqueColor::new([
            color.red() as f32,
            color.green() as f32,
            color.blue() as f32,
        ]);

        let hsv: OpaqueColor<Hsv> = col.convert();

        self.set_h(hsv.components[0]);
        self.set_s(hsv.components[1]);
        self.set_v(hsv.components[2]);
    }

    fn pick_color(&self) {
        glib::spawn_future_local(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            async move {
                match request_picker().await {
                    Ok(color) => obj.set_color(color),
                    Err(e) => eprintln!("[ Warn ] {}", e),
                }
            }
        ));
    }

    fn copy_color(&self, content: Option<String>) {
        let clipboard = self.clipboard();
        let overlay = &self.imp().toast_overlay;

        let hsv: OpaqueColor<Hsv> = OpaqueColor::new([self.h(), self.s(), self.v()]);

        let [r, g, b, _] = hsv.to_rgba8().to_u8_array();
        let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);

        let text = content.unwrap_or(hex);
        clipboard.set_text(&text);

        let toast = adw::Toast::builder()
            .title("Copied color")
            .timeout(1)
            .build();

        overlay.add_toast(toast);
    }

    fn setup_boxes(&self) {
        let imp = self.imp();
        let list = &imp.list;

        for format in ColorFormat::iter() {
            let color_box = PiccoloColorBox::new(format);

            self.bind_property("h", &color_box, "h")
                .sync_create()
                .build();
            self.bind_property("s", &color_box, "s")
                .sync_create()
                .build();
            self.bind_property("v", &color_box, "v")
                .sync_create()
                .build();

            list.append(&color_box);
        }
    }
}

async fn request_picker() -> ashpd::Result<Color> {
    Ok(Color::pick().send().await?.response()?)
}

#[derive(strum::Display, strum::AsRefStr, strum::EnumIter)]
pub enum WindowAction {
    #[strum(to_string = "win.copy-color")]
    ColorCopy,
    #[strum(to_string = "win.pick-color")]
    ColorPick,
    #[strum(to_string = "win.clear-focus")]
    FocusClear,
}

impl Deref for WindowAction {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl WindowAction {
    fn init_actions(klass: &mut <imp::PiccoloWindow as ObjectSubclass>::Class) {
        for action in Self::iter() {
            match action {
                Self::ColorCopy => {
                    klass.install_action(&action, Some(VariantTy::STRING), |win, _, arg| {
                        // This action should take the string with the color name or css function and
                        // copy it to the clipboard
                        if let Some(var) = arg {
                            let value = var.to_string(); // 'color'
                            let text = value.get(1..value.len().sub(1)).unwrap(); // Remove quotes
                            if !text.is_empty() {
                                win.copy_color(Some(text.to_string()));
                            } else {
                                win.copy_color(None);
                            }
                        }
                    });
                }
                Self::ColorPick => {
                    klass.install_action(&action, None, |win, _, _| {
                        win.pick_color();
                    });

                    klass.add_binding_action(
                        gdk::Key::P,
                        gdk::ModifierType::NO_MODIFIER_MASK,
                        &action,
                    );
                }
                Self::FocusClear => {
                    klass.install_action(&action, None, |win, _, _| {
                        win.clear_focus();
                    });

                    klass.add_binding_action(
                        gdk::Key::Escape,
                        gdk::ModifierType::NO_MODIFIER_MASK,
                        &action,
                    );
                }
            }
        }
    }
}
