/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
use crate::prelude::*;

use adw::TabView;
use gtk::glib::{ParamFlags, ParamSpec, ParamSpecObject, Value};
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;
use libchidna::vertical_tab_bar::VerticalTabBar;
use once_cell::sync::Lazy;
use std::cell::RefCell;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/io/fortressia/Echidna/sidebar.ui")]
pub struct EchidnaSidebar {
    #[template_child]
    vtb_bin: TemplateChild<gtk::Box>,
    tab_view: RefCell<TabView>,
}

#[glib::object_subclass]
impl ObjectSubclass for EchidnaSidebar {
    const NAME: &'static str = "EchidnaSidebar";
    type Type = super::EchidnaSidebar;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for EchidnaSidebar {
    fn constructed(&self, _obj: &Self::Type) {
        self.vtb_bin
            .append(&VerticalTabBar::new(&self.tab_view.borrow().clone()));
    }

    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![ParamSpecObject::new(
                "tab-view",
                "tab-view",
                "the tab view of the editor",
                TabView::static_type(),
                ParamFlags::READWRITE,
            )]
        });

        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, spec: &ParamSpec) {
        match spec.name() {
            "tab-view" => {
                let view: TabView = value.get().expect("The tab_view needs to be AdwTabView");
                self.tab_view.replace(view);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, spec: &ParamSpec) -> Value {
        match spec.name() {
            "tab-view" => self.tab_view.borrow().to_value(),
            _ => unimplemented!(),
        }
    }
}
impl WidgetImpl for EchidnaSidebar {}
impl BoxImpl for EchidnaSidebar {}
