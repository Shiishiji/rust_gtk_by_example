use std::cell::Cell;
use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, template_callbacks};

use crate::custom_button::CustomButton;

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/shiishiji/my-gtk-app/window.ui")]
pub struct Window {
    pub number: Cell<i32>
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "MyGtkAppWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        CustomButton::ensure_type();

        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

#[gtk::template_callbacks]
impl Window {
    #[template_callback]
    fn handle_button_clicked(&self, button: &CustomButton) {
        let number_increased = self.number.get() + 1;
        self.number.set(number_increased);
        println!("Button with label {} clicked.", button.number().to_string());
        println!("Widget internal number increased to {}", number_increased);
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Window {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();
    }
}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}