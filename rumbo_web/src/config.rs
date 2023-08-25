use std::env;
pub fn get_debug_config() -> ConfigValues {
    ConfigValues {
        static_files_path: "./target/debug/static",
        host_address: "localhost",
        port: 8081,

        db_url: "postgres://postgres:example@localhost/rumbo_app".to_string()
    }
}

pub fn get_production_config() -> ConfigValues {
    let connection_string: String =
        env::var("DATABASE_URL").unwrap_or("postgres://postgres:example@db/rumbo_app".to_string());

    ConfigValues {
        static_files_path: "./static/",
        host_address: "0.0.0.0",
        port: 8081,

        db_url: connection_string
    }
}
pub struct ConfigValues {
    pub static_files_path: &'static str,
    pub host_address: &'static str,
    pub port: u16,

    pub db_url: String
}
