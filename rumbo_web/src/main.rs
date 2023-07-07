use actix_files as fs;
use actix_web::{middleware, App, HttpServer};
use std::io::Write;
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use std::env;

mod config;
mod metric_service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let is_production = env::var("IS_PRODUCTION").is_ok();
    let config = match is_production {
        true => config::get_production_config(),
        _ => config::get_debug_config()
    };

    Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Trace)
        .init();
    

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(metric_service::get_metric)
            .service(fs::Files::new("/", config.static_files_path).index_file("index.html"))    
    })
    .bind((config.host_address, config.port))?
    .run()
    .await
}
