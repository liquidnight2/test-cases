/**   Needs:
#  apt-get install  libgraphene-1.0-dev
#  apt-get install  librust-gdk4-sys-dev
*/
use glib::clone;

use gtk::glib;
use gtk::prelude::*;

fn on_activate(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    let button = gtk::Button::with_label("Hello World!  gtk4 ");
    button.connect_clicked(clone!(@weak window => move |_| window.close()));
    window.set_child(Some(&button));
    window.present();
}

fn main() {
    let app = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.basic")
        .build();
    app.connect_activate(on_activate);
    app.run();
}
