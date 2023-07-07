pub fn get_debug_config() -> ConfigValues {
    ConfigValues {
        static_files_path: "./target/debug/static",
        host_address: "localhost",
        port: 8080
    }
}

pub fn get_production_config() -> ConfigValues {
    ConfigValues {
        static_files_path: "./static/",
        host_address: "0.0.0.0",
        port: 8080
    }
}

pub struct ConfigValues {
    pub static_files_path: &'static str,
    pub host_address: &'static str,
    pub port: u16
}