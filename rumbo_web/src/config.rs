use std::env;
pub fn get_debug_config() -> ConfigValues {
    ConfigValues {
        static_files_path: "./target/debug/static",
        host_address: "localhost",
        port: 8081,

        db_url: "postgres://postgres:example@localhost/rumbo_app".to_string(),
        token_cache_connection: "redis://redis/".to_string()
    }
}

pub fn get_production_config() -> ConfigValues {
    let connection_string: String =
        env::var("DATABASE_URL").unwrap_or("postgres://postgres:example@db/rumbo_app".to_string());
    
    let token_cache_connection = 
        env::var("TOKEN_CACHE_URL").unwrap_or("redis://redis/".to_string());

    ConfigValues {
        static_files_path: "./static/",
        host_address: "0.0.0.0",
        port: 8081,

        db_url: connection_string,
        token_cache_connection: token_cache_connection
    }
}
pub struct ConfigValues {
    pub static_files_path: &'static str,
    pub host_address: &'static str,
    pub port: u16,

    pub db_url: String,
    pub token_cache_connection: String
}
