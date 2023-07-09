use actix_files as fs;
use actix_web::{middleware, App, HttpServer, web};
use chrono::Local;
use config::ConfigValues;
use env_logger::Builder;
use log::{LevelFilter};

use rumbo_logic::*;
use std::env;
use std::io::Write;

mod config;
mod metric_service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    build_logger();

    let config = get_config();
    let app_sate = get_app_state(&config).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_sate.clone()))

            .wrap(middleware::Logger::default())

            .service(metric_service::get_metric)
            .service(metric_service::create_metric)
            .service(metric_service::delete_metric)
            .service(metric_service::update_metric)
            
            .service(fs::Files::new("/", config.static_files_path).index_file("index.html"))
    })
    .bind((config.host_address, config.port))?
    .run()
    .await
}

async fn get_app_state(config: &ConfigValues) -> Result<RumboApp> {
    let app = RumboApp::new(&config.mongo_host, config.mongo_app_name).await?;

    Ok(app)
}

fn build_logger() {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Trace)
        .init();
}

fn get_config() -> ConfigValues {
    let is_production = env::var("IS_PRODUCTION").is_ok();
    match is_production {
        true => config::get_production_config(),
        _ => config::get_debug_config(),
    }
}
