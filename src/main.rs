mod custom_button;
mod custom_window;
mod integer_object;

use std::thread;
use std::time::Duration;
use custom_button::CustomButton;
use gtk::prelude::*;
use gtk::{glib, Application, Box, Switch, Align, Orientation, Button, gio, SignalListItemFactory, Label, ListItem, SingleSelection, ListView, PolicyType, ScrolledWindow, Widget, CustomFilter, FilterListModel, SortListModel, CustomSorter, FilterChange, SorterChange, StringList, StringObject, NoSelection};
use gtk::gio::Settings;
use gtk::glib::{clone, closure_local, MainContext, Priority};
use crate::custom_window::Window;
use crate::integer_object::IntegerObject;

const APP_ID: &str = "com.shiishiji.my-gtk.app";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let settings = Settings::new(&*format!("{}.Settings1", APP_ID));
    let gtk_box = Box::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .valign(Align::Center)
        .halign(Align::Center)
        .spacing(12)
        .orientation(Orientation::Vertical)
        .build();

    let button = CustomButton::new();
    button.set_margin_top(12);
    button.set_margin_bottom(12);
    button.set_margin_start(12);
    button.set_margin_end(12);

    button.connect_number_notify(|button|{
       println!("{}", button.number());
    });

    button.connect_closure(
        "max-number-reached",
        false,
        closure_local!(move |_button: CustomButton, number: i32| {
            println!("Max number reached! {}", number);
        }),
    );

    let s1 = Switch::new();
    let s2 = Switch::new();

    settings
        .bind("is-switch-enabled", &s1, "active")
        .build();

    settings
        .bind("is-switch-enabled", &s2, "active")
        .build();

    // let is_switch_enabled = settings.boolean("is-switch-enabled");
    // s1
    //     .bind_property("active", &s2, "active")
    //     .bidirectional()
    //     .build();
    //
    // s1.connect_state_set(move |_, is_enabled| {
    //     settings
    //         .set_boolean("is-switch-enabled", is_enabled)
    //         .expect("Could not set setting.");
    //     glib::Propagation::Proceed
    // });


    // ----------------
    // < przyciski ze zbindownym property
    // ----------------

    let b1 = CustomButton::new();
    let b2 = CustomButton::new();

    b1.bind_property("number", &b2, "number")
        .transform_to(|_, number: i32| {
            let incremented_number = number + 1;
            Some(incremented_number.to_value())
        })
        .transform_from(|_, number: i32| {
            let decremented_number = number - 1;
            Some(decremented_number.to_value())
        })
        .bidirectional()
        .sync_create()
        .build();

    // ----------------
    // < przyciski ze zbindownym property
    // ----------------

    // ----------------
    // < przycisk blokujący wątek
    // ----------------

    let b_block = Button::new();
    b_block.set_label("Block thread for 5");

    let (sender, receiver) = MainContext::channel(Priority::default());
    b_block.connect_clicked(move |_b: &Button| {
        println!{"CLICKED BLOCKING BUTTON"};
        let sender = sender.clone();
        gio::spawn_blocking(move || {
            sender.send(false).expect("could not send through channel");
            let five_seconds = Duration::from_secs(5);
            thread::sleep(five_seconds);
            sender.send(true).expect("could not send through channel");
        });
    });

    // ----------------
    // > przycisk blokujący wątek
    // ----------------

    // ----------------
    // < lista stringów
    // ----------------

    let model_str: StringList = (0..=100_000).map(|number| number.to_string()).collect();
    let factory_str = SignalListItemFactory::new();

    factory_str.connect_setup(move |_, list_item| {
        let label = Label::new(None);
        let list_item = list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem");
        list_item.set_child(Some(&label));

        list_item
            .property_expression("item")
            .chain_property::<StringObject>("string")
            .bind(&label, "label", Widget::NONE);
    });

    let selection_model_str = NoSelection::new(Some(model_str));
    let list_view_str = ListView::new(Some(selection_model_str), Some(factory_str));

    let scrolled_window_str = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .min_content_width(360)
        .child(&list_view_str)
        .build();

    // ----------------
    // > lista stringów
    // ----------------


    // ----------------
    // < lista wybierana
    // ----------------

    let vector: Vec<IntegerObject> = (0..=100_000).map(IntegerObject::new).collect();
    let model = gio::ListStore::new::<IntegerObject>();
    model.extend_from_slice(&vector);

    let factory = SignalListItemFactory::new();
    factory.connect_setup(move |_, list_item| {
        let label = Label::new(None);
        list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .set_child(Some(&label));

        list_item
            .property_expression("item")
            .chain_property::<IntegerObject>("number")
            .bind(&label, "label", Widget::NONE);
    });

    let filter = CustomFilter::new(move |obj| {
        let integer_object = obj
            .downcast_ref::<IntegerObject>()
            .expect("The object needs to be of type `IntegerObject`.");

        integer_object.number() % 2 == 0
    });
    let filter_model = FilterListModel::new(Some(model), Some(filter.clone()));
    let sorter = CustomSorter::new(move |obj1, obj2| {
        // Get `IntegerObject` from `glib::Object`
        let integer_object_1 = obj1
            .downcast_ref::<IntegerObject>()
            .expect("The object needs to be of type `IntegerObject`.");
        let integer_object_2 = obj2
            .downcast_ref::<IntegerObject>()
            .expect("The object needs to be of type `IntegerObject`.");

        // Get property "number" from `IntegerObject`
        let number_1 = integer_object_1.number();
        let number_2 = integer_object_2.number();

        // Reverse sorting order -> large numbers come first
        number_2.cmp(&number_1).into()
    });
    let sort_model = SortListModel::new(Some(filter_model), Some(sorter.clone()));

    let selection_model = SingleSelection::new(Some(sort_model));
    let list_view = ListView::new(Some(selection_model), Some(factory));

    list_view.connect_activate(move |list_view, position| {
        let model = list_view.model().expect("The model has to exist.");
        let integer_object = model
            .item(position)
            .and_downcast::<IntegerObject>()
            .expect("The item has to be an `IntegerObject`.");

        integer_object.increase_number();

        filter.changed(FilterChange::Different);
        sorter.changed(SorterChange::Different);
    });

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .min_content_width(360)
        .child(&list_view)
        .build();


    // ----------------
    // > lista wybierana
    // ----------------

    gtk_box.append(&scrolled_window_str);
    gtk_box.append(&button);
    gtk_box.append(&s1);
    gtk_box.append(&s2);
    gtk_box.append(&b1);
    gtk_box.append(&b2);
    gtk_box.append(&b_block);
    gtk_box.append(&scrolled_window);

    receiver.attach(
        None,
        clone!(@weak button => @default-return glib::ControlFlow::Break,
            move |enable_button| {
                b_block.set_sensitive(enable_button);
                glib::ControlFlow::Continue
            }
        ),
    );

    let window = Window::new(app);
    window.set_title(Some("Moja apka"));
    window.set_child(Some(&gtk_box));

    window.present();
}
