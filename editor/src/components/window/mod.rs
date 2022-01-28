/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

pub mod file;
pub mod imp;
pub mod menubar;

use glib::object::{Cast, IsA};
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct EchidnaWindow(ObjectSubclass<imp::EchidnaWindow>)
    @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
    @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl EchidnaWindow {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        let object = glib::Object::new(&[("application", &application)]);

        match object {
            Ok(o) => o,
            Err(e) => panic!("Error in making EchidnaApplication {}", e),
        }
    }

    pub fn to_imp(&self) -> &imp::EchidnaWindow {
        imp::EchidnaWindow::from_instance(self)
    }

    pub fn get_current_tab<A: IsA<gtk::Widget>>(&self) -> Result<A, &str> {
        let window_imp = self.to_imp();
        let nth = window_imp.notebook.current_page();

        match nth {
            None => Err("No tabs are currently opened, maybe there are no tabs."),
            Some(nth) => {
                let page = window_imp
                    .notebook
                    .nth_page(Some(nth))
                    .expect("Couldn't get the page of the current index.");

                match page.downcast::<A>()
                {
                    Ok(page) => Ok(page),
                    Err(e) => Err("Cannot downcast to type parameter A. Maybe it's not in the type you are looking for."),
                }
            }
        }
    }
}