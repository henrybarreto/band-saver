//! GTK application what uses the 'wandershaper' script what uses some networking tools to limit the bandwidth and helping saving data.

use std::rc::Rc;

use gtk::prelude::*;
use gio::prelude::*;

use gtk::Application;
use wondershaper::Wondershaper;

mod wondershaper;

fn main() {
    // let wondershaper: Wondershaper = Wondershaper::new("./config/wondershaper.conf".to_string());
    let interfaces: Rc<Vec<String>> = Rc::new(Wondershaper::get_network_interfaces_name());
    // let wondershaper_config = Wondershaper::create_cofiguration_file("./config/wondershaper.conf", wondershaper_config);
    gtk::init();
    let app = Application::builder()
        .application_id("dev.henrybarreto.bandwidth-saving")
        .build();

    let builder = gtk::Builder::from_file("./config/ui.glade"); 

    app.connect_activate(move |app| {
        let win_main: gtk::Window = builder.object("win_main")
            .expect("Could not find the win_main in the ui file");
        let cbb_network_intefaces: gtk::ComboBoxText = builder.object("cbb_network_interfaces")
            .expect("Could not find the cbb_network_interfaces in the ui file");
        let btn_apply: gtk::Button = builder.object("btn_apply")
            .expect("Could not find the btn_apply in the ui file");
        let et_downlaod_speed: gtk::Entry = builder.object("et_downlaod_speed")
            .expect("Could not find the et_download_speed in the ui file");
        let et_upload_speed: gtk::Entry = builder.object("et_upload_speed")
            .expect("Could not find the et_upload_speed in the ui file");
        // ---

        let insert_data_combo_box_text = |combo_box_text: &gtk::ComboBoxText, vec_data: Vec<String>| {
            vec_data
            .iter()
            .enumerate().
            inspect(|(index, data)| {
                combo_box_text.insert_text(*index as i32, *data);
            }).map(|(_, data)| {data})
            .for_each(drop);
        };

        insert_data_combo_box_text(&cbb_network_intefaces, interfaces.clone().to_vec());
        win_main.show_all();
    });

    app.run();
    gtk::main();
}

        // interfaces
        // .iter()
        // .enumerate().
        // inspect(|(index, name)| {
        //     cbb_network_intefaces.insert_text(*index as i32, name);
        // }).map(|(_, name)| {name})
        // .for_each(drop);

        // btn_apply.connect(move |button| {
        //     gtk::main_quit();
        // });
