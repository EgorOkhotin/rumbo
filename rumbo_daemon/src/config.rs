extern crate serde_yaml;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use serde_yaml::Result;


#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig{
    first_metric: Option<String>,
    second_metrick: Option<SecondMetric>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecondMetric{
    value: Option<String>
}

pub fn read_config(str:String) -> Result<AppConfig>{
    let node:AppConfig = serde_yaml::from_str(&str)?;
    println!("{:?}", node);
    Ok(node)
}

pub fn read_config_file()->String{
    use std::fs;

    let path_to_config = String::from("config.yml");

    return fs::read_to_string(path_to_config).unwrap();
}

