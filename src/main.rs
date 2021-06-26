//! GTK application what uses the 'wandershaper' script what uses some networking tools to limit the bandwidth and helping saving data.

use gtk::prelude::*;
use gio::prelude::*;

use gtk::Application;

mod wondershaper;

fn main() {
    gtk::init();
    let app = Application::builder()
        .application_id("dev.henrybarreto.bandwidth-saving")
        .build();

    let builder = gtk::Builder::from_file("./config/ui.glade"); 

    app.connect_activate(move |app| {
        // We create the main window.
        let win: gtk::Window = builder.object("win_main").unwrap();

        // Don't forget to make all widgets visible.
        win.show();
    });

    app.connect_shutdown(|app| {
        gtk::main_quit();
    });

    app.run();
    gtk::main();
}
