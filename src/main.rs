mod window;
mod custom_button;

use gtk::prelude::*;
use gtk::{glib, Application, Box, Switch, Align, Orientation, Button, gio, SignalListItemFactory, Label, ListItem, SingleSelection, ListView, PolicyType, ScrolledWindow, Widget, CustomFilter, FilterListModel, SortListModel, CustomSorter, FilterChange, SorterChange, StringList, StringObject, NoSelection};
use window::Window;

const APP_ID: &str = "com.shiishiji.my-gtk.app";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("gresource")
        .expect("Failed to register resources");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let window = Window::new(app);
    window.present();
}