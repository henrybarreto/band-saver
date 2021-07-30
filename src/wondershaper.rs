use pnet::datalink::{NetworkInterface, interfaces};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Read, Write};
use std::error::Error;

/**
  Structure with the main methods to perfom action in the wondershaper script
*/
pub struct Wondershaper {
  pub wondershaper_config: WondershaperConfig
}

impl Wondershaper {
  pub fn new(configuration_file_path: String) -> Self {
    Wondershaper {
      wondershaper_config: Wondershaper::load_configuration_file(configuration_file_path)
    }
  }
  pub fn get_network_interfaces_name() -> Vec<String> {
    interfaces().iter()
    .map(|interface| interface.name.to_owned())
    .collect::<Vec<String>>()
  }

  fn bandwidth_save(inteface: String, download_speed: String, upload_speed: String) -> Result<(), Box<dyn Error>> {
    // TODO Fazer a implementção principal da aplicação
    Ok(())
  }

  /**
    Loading a wondershaper.conf file from a path and return a WondershaperConfig.

    If the file could not be open, the application will panic.
  */
  fn load_configuration_file(configuration_file_path: String) -> WondershaperConfig {
    let mut wondershaper_file: File = File::open(configuration_file_path)
      .expect("Could not find the wondershaper.conf file");
    let mut data_from_wondershaper_file: Vec<u8> = vec![];
    let _bytes_read_from_wondershaper_file: usize = wondershaper_file
      .read_to_end(&mut data_from_wondershaper_file).expect("Could not read the wondershaper.conf file");
    let config_from_wondershaper_file = toml::from_slice::<WondershaperConfigFile>(&mut data_from_wondershaper_file)
      .expect("Could not deserialize the wondershaper.conf file");
    config_from_wondershaper_file.wondershaper
  }
  /**
    Creating a wondershaper.conf file to a path and return a Vec<u8> with the bytes wrote..

    If the file could not be created, the application will panic.
  */
  pub fn create_cofiguration_file(configuration_file_path: String, wondershaper_config: &WondershaperConfigFile) -> Vec<u8>{
    let mut wondershaper_file = File::create(configuration_file_path)
      .expect("Could not create the configuration file");
    let wondershaper_file_string= toml::to_string(
        wondershaper_config
      ).expect("Could not convert the wodershaper config file to toml");
    let wondershaper_file_bytes = wondershaper_file_string.as_bytes();
    wondershaper_file.write_all(
      &wondershaper_file_bytes
    ).expect("Could not write to the wondershaper configuration file");

    return wondershaper_file_bytes.to_vec()
  }
}

/**
  Labels accepted for wondershaper configuration file
*/
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WondershaperConfig {
    IFACE: String,
    DSPEED: String,
    USPEED: String,
    HIPRIODST: Option<String>,
    COMMONOPTIONS: Option<String>,
    NOPRIOHOSTSRC: Option<u16>,
    NOPRIOHOSTDST: Option<String>,
    NOPRIOPORTSRC: Option<String>,
    NOPRIOPORTDST: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WondershaperConfigFile {
    wondershaper: WondershaperConfig
}


#[cfg(test)]
mod wondershaper_test {
  use super::{WondershaperConfigFile, Wondershaper, WondershaperConfig};
  #[test]
  fn create_wondershaper_file_test() {
    let wondershaper_config = WondershaperConfig {
      IFACE: "wlan0".to_string(),
      DSPEED: "1024".to_string(),
      USPEED: "1024".to_string(),
      HIPRIODST: Option::None,
      COMMONOPTIONS: Option::None,
      NOPRIOHOSTSRC: Option::None,
      NOPRIOHOSTDST: Option::None,
      NOPRIOPORTSRC: Option::None,
      NOPRIOPORTDST: Option::None,
    };
    let wondershaper_config_file = WondershaperConfigFile {
      wondershaper: wondershaper_config
    };

    Wondershaper::create_cofiguration_file(
      "config/wondershaper.conf".to_string(), 
      &wondershaper_config_file);
  }
  fn create_wondershaper_file_with_real_interfaace_test() {
    let interfaces = Wondershaper::get_network_interfaces_name();
    let wondershaper_config = WondershaperConfig {
      IFACE: interfaces[1].clone(),
      DSPEED: "1024".to_string(),
      USPEED: "1024".to_string(),
      HIPRIODST: Option::None,
      COMMONOPTIONS: Option::None,
      NOPRIOHOSTSRC: Option::None,
      NOPRIOHOSTDST: Option::None,
      NOPRIOPORTSRC: Option::None,
      NOPRIOPORTDST: Option::None,
    };
    let wondershaper_config_file = WondershaperConfigFile {
      wondershaper: wondershaper_config
    };

    Wondershaper::create_cofiguration_file(
      "config/wondershaper.conf".to_string(), 
      &wondershaper_config_file);
  }

  #[test]
  fn open_wondershaper_file_test() {
    Wondershaper::new(String::from("config/wondershaper.conf"));
  }
}