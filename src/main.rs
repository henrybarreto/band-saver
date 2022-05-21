use gtk4 as gtk;
use std::{rc::Rc};
use gio::{glib::GString, prelude::*};
use gtk::prelude::*;
use gtk::Application;
use logs::{debug, error, trace};
use wondershaper::{Wondershaper, WondershaperConfig, WondershaperConfigFile};

mod wondershaper;

fn main() {
    let interfaces: Rc<Vec<String>> = Rc::new(Wondershaper::get_interfaces());

    gtk::init().expect("Failed to initialize GTK.");

    let app = Application::new(
        Some("dev.henrybarreto.band-saver"),
        Default::default(),
    );

    let builder = gtk::Builder::from_file("assets/band-saver.cmb");

    app.connect_activate(move |app| {
        let set_combobox_data =
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

        let get_combobox_data = |combo_box_text: &gtk::ComboBoxText| {
            combo_box_text
                .active_text()
                .unwrap_or(GString::from(""))
                .to_string()
        };

        let get_entry_data = |entry: &gtk::Entry| entry.text().to_string();

        trace!("Loading the UI components from ui's file");

        let win_main: gtk::Window = builder
            .object("win_main")
            .expect("Could not find the win_main in the ui file");
        let cbb_network_intefaces: gtk::ComboBoxText = builder
            .object("cbb_network_interfaces")
            .expect("Could not find the cbb_network_interfaces in the ui file");
        let btn_apply: gtk::Button = builder
            .object("btn_apply")
            .expect("Could not find the btn_apply in the ui file");
        let btn_reset: gtk::Button = builder
            .object("btn_reset")
            .expect("Could not find the btn_reset in the ui file");
        let et_downlaod_speed: gtk::Entry = builder
            .object("et_downlaod_speed")
            .expect("Could not find the et_download_speed in the ui file");
        let et_upload_speed: gtk::Entry = builder
            .object("et_upload_speed")
            .expect("Could not find the et_upload_speed in the ui file");

        trace!("UI components loaded");

        let cbb_network_intefaces_clone = cbb_network_intefaces.clone();
        // btn_apply.connect_button_press_event(move |_btn, _event| {
        btn_apply.connect_clicked(move |_| {
            trace!("Getting the data from the UI");

            let interface_selected = get_combobox_data(&cbb_network_intefaces_clone);
            let download_speed = get_entry_data(&et_downlaod_speed);
            let upload_speed = get_entry_data(&et_upload_speed);

            trace!("Data got from the UI");

            let wondershaper_config =
                WondershaperConfig::new(interface_selected, download_speed, upload_speed);
            let wondershaper_config_file = WondershaperConfigFile {
                wondershaper: wondershaper_config,
            };

            Wondershaper::create_cofiguration_file(
                "/etc/systemd/wondershaper.conf".to_string(),
                &wondershaper_config_file,
            );

            let cmd_child = Wondershaper::apply();
            if let Ok(child) = cmd_child {
                debug!("Wondershaper apply command executed");
                trace!("{:?}", child.wait_with_output());
            } else {
                error!("{:#?}", "Could not execute the wondershaper stop command");
            }

            // gtk::Inhibit(false)
        });

        btn_reset.connect_clicked(move |_| {
            let cmd_child = Wondershaper::reset();
            if let Ok(child) = cmd_child {
                debug!("Wondershaper stop command executed");
                trace!("{:?}", child.wait_with_output());
            } else {
                error!("{:#?}", "Could not execute the wondershaper stop command");
            }

        });

        trace!("Connecting the signals to the UI components");

        win_main.connect_destroy(|_| {
            Wondershaper::reset().expect("Could not reset the wondershaper limits");
            // gtk::main_quit();
        });

        trace!("Setting the data to the UI components");

        set_combobox_data(&cbb_network_intefaces, interfaces.clone().to_vec());

        trace!("Trying to show all the UI components");

        win_main.show();

        trace!("Showed the main window");
    });

    app.run();
}
