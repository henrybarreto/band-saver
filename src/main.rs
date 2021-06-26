//! GTK application what uses the 'wandershaper' script what uses some networking tools to limit the bandwidth and helping saving data.

mod wondershaper;
use wondershaper::Wondershaper;

fn main() {
    let wondershaper = Wondershaper::new(String::from("config/wondershaper.conf"));
    print!("{:#?}", wondershaper.wondershaper_config);

}
