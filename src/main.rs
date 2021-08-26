//! GTK application what uses the 'wandershaper' script what uses some networking tools to limit the bandwidth and helping saving data.

use std::{process, rc::Rc};

use gio::{glib::GString, prelude::*};
use gtk::prelude::*;

use gtk::Application;
use log::{debug, error, info};
use simple_logger::SimpleLogger;
use wondershaper::{Wondershaper, WondershaperConfig, WondershaperConfigFile};

mod wondershaper;

fn main() {
    SimpleLogger::new().init().unwrap();
    // let wondershaper: Wondershaper = Wondershaper::new("./config/wondershaper.conf".to_string());
    let interfaces: Rc<Vec<String>> = Rc::new(Wondershaper::list_network_interfaces_name());
    // let wondershaper_config = Wondershaper::create_cofiguration_file("./config/wondershaper.conf", wondershaper_config);
    gtk::init().expect("Could not init the gtk");
    let app = Application::builder()
        .application_id("dev.henrybarreto.bandwidth-saving")
        .build();

    let builder = gtk::Builder::from_file("./config/ui.glade");

    app.connect_activate(move |app| {
        let win_main: gtk::Window = builder
            .object("win_main")
            .expect("Could not find the win_main in the ui file");
        let cbb_network_intefaces: gtk::ComboBoxText = builder
            .object("cbb_network_interfaces")
            .expect("Could not find the cbb_network_interfaces in the ui file");
        let btn_apply: gtk::Button = builder
            .object("btn_apply")
            .expect("Could not find the btn_apply in the ui file");
        let et_downlaod_speed: gtk::Entry = builder
            .object("et_downlaod_speed")
            .expect("Could not find the et_download_speed in the ui file");
        let et_upload_speed: gtk::Entry = builder
            .object("et_upload_speed")
            .expect("Could not find the et_upload_speed in the ui file");
        // ---
        let insert_data_combo_box_text =
            |combo_box_text: &gtk::ComboBoxText, vec_data: Vec<String>| {
                vec_data
                    .iter()
                    .enumerate()
                    .inspect(|(index, data)| {
                        combo_box_text.insert_text(*index as i32, *data);
                    })
                    .map(|(_, data)| data)
                    .for_each(drop);
            };
        let capture_combo_box_text_data = |combo_box_text: &gtk::ComboBoxText| {
            combo_box_text
                .active_text()
                .unwrap_or(GString::from(""))
                .to_string()
        };

        let capture_entry_data = |entry: &gtk::Entry| entry.text().to_string();

        let cbb_network_intefaces_clone = cbb_network_intefaces.clone();
        btn_apply.connect_button_press_event(move |_btn, _event| {
            let interface_selected = capture_combo_box_text_data(&cbb_network_intefaces_clone);
            let download_speed = capture_entry_data(&et_downlaod_speed);
            let upload_speed = capture_entry_data(&et_upload_speed);
            // let wondershaper = Wondershaper::new("./config/wondershaper.conf".to_string());
            let wondershaper_config =
                WondershaperConfig::new(interface_selected, download_speed, upload_speed);
            let wondershaper_config_file = WondershaperConfigFile {
                wondershaper: wondershaper_config,
            };
            Wondershaper::create_cofiguration_file(
                "/etc/systemd/wondershaper.conf".to_string(),
                &wondershaper_config_file,
            );
            let cmd_child = Wondershaper::run();
            match cmd_child {
                Ok(child) => {
                    info!("Wondershaper command executed");
                    debug!("{:?}", child.wait_with_output());
                }
                Err(err) => {
                    error!("{:#?}", err);
                }
            }
            Inhibit(false)
        });
        win_main.connect_destroy(|_| {
            gtk::main_quit();
        });

        insert_data_combo_box_text(&cbb_network_intefaces, interfaces.clone().to_vec());
        win_main.show_all();
    });

    app.run();
    gtk::main();
}
