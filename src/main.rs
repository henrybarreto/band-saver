//! GTK application what uses the 'wandershaper' script what uses some networking tools to limit the bandwidth and helping saving data.

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

mod wondershaper;

fn main() {
    let builder = gtk::Builder::from_file(""); 
    let app = Application::builder()
        .application_id("dev.henrybarreto.bandwidth-saving")
        .build();

    app.connect_activate(|app| {
        // We create the main window.
        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Hello, World!")
            .build();

        // Don't forget to make all widgets visible.
        win.show_all();
    });
    app.run();
}
