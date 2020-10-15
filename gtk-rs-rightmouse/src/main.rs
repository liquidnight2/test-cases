//! # TreeView Sample
//! modified by connect_button_press_event
//!

extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::{
    ApplicationWindow, CellRendererText, Label, ListStore, Orientation, TreeView, TreeViewColumn,
    WindowPosition,
};

use std::env::args;

fn create_and_fill_model() -> ListStore {
    let model = ListStore::new(&[u32::static_type(), String::static_type()]);
    let entries = &["Michel", "Sara", "Liam", "Zelda", "Neo", "Octopus master"];
    for (i, entry) in entries.iter().enumerate() {
        model.insert_with_values(None, &[0, 1], &[&(i as u32 + 1), &entry]);
    }
    model
}

fn append_column(tree: &TreeView, id: i32) {
    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();
    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", id);
    tree.append_column(&column);
}

fn create_and_setup_view() -> TreeView {
    let tree = TreeView::new();
    tree.set_headers_visible(false);
    append_column(&tree, 0);
    append_column(&tree, 1);
    tree
}

fn build_ui(application: &gtk::Application) {
    let window = ApplicationWindow::new(application);
    window.set_title("Simple TreeView example");
    window.set_position(WindowPosition::Center);
    let vertical_layout = gtk::Box::new(Orientation::Vertical, 0);
    let label = Label::new(None);
    let tree = create_and_setup_view();
    let model = create_and_fill_model();
    tree.set_model(Some(&model));
    vertical_layout.add(&tree);
    vertical_layout.add(&label);

    tree.connect_cursor_changed(move |tree_view| {
        let selection = tree_view.get_selection();
        if let Some((model, iter)) = selection.get_selected() {
            let val = model.get_value(&iter, 1).get::<String>().unwrap();
            let idx = model.get_value(&iter, 0).get_some::<u32>().unwrap();
            label.set_text(&format!("Hello '{:?}' from row {:?}", val, idx));
            println!("cursor_changed '{:?}' from row {:?}", val, idx);
        }
    });

    tree.connect_button_press_event(move |p_tv, eventbutton| {
        let treeview: gtk::TreeView = p_tv.clone().dynamic_cast::<gtk::TreeView>().unwrap();
        let button_num = eventbutton.get_button() as i32;
        let selection = treeview.get_selection();
        if let Some((model, iter)) = selection.get_selected() {
            let val = model.get_value(&iter, 1).get::<String>().unwrap();
            let idx = model.get_value(&iter, 0).get_some::<u32>().unwrap();
            println!(
                "button_press_event  button{} '{:?}' from row {:?}",
                button_num, val, idx
            );
        }
        gtk::Inhibit(false)
    });

    window.add(&vertical_layout);
    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.simple_treeview"),
        Default::default(),
    )
    .expect("Initialization failed...");
    application.connect_activate(|app| {
        build_ui(app);
    });
    application.run(&args().collect::<Vec<_>>());
}
