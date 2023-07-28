mod config;
use std::collections::BTreeMap;

use crate::config::AppConfig;

fn main() {
    let config_property:String = config::read_config_file();
    println!("111111111111");
      println!("{}", config_property);
    let properties:AppConfig=config::read_config(config_property).unwrap();
    // print!(properties)
}

