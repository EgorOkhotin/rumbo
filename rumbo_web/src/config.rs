use std::env;
pub fn get_debug_config() -> ConfigValues {
    ConfigValues {
        static_files_path: "./target/debug/static",
        host_address: "localhost",
        port: 8080,

        mongo_app_name: &APP_NAME,
        mongo_host: "mongodb://root:example@localhost:27017/".to_string(),
    }
}

pub fn get_production_config() -> ConfigValues {
    let mongo_host: String =
        env::var("MONGO_HOST").unwrap_or("mongodb://root:example@mongo:27017/".to_string());

    ConfigValues {
        static_files_path: "./static/",
        host_address: "0.0.0.0",
        port: 8080,

        mongo_app_name: &APP_NAME,
        mongo_host: mongo_host,
    }
}

const APP_NAME: &'static str = "RUMBO";

pub struct ConfigValues {
    pub static_files_path: &'static str,
    pub host_address: &'static str,
    pub port: u16,

    pub mongo_host: String,
    pub mongo_app_name: &'static str,
}
