mod prelude {
    pub use super::config::ConfigValues;
    pub use actix_files as fs;
    pub use actix_web::{
        delete, get, http::header::ContentType, middleware, patch, post, web, App, HttpResponse,
        HttpServer, Responder,
    };
    pub use chrono::Local;
    pub use env_logger::Builder;
    pub use log::{info, LevelFilter};
    pub use std::env;
    pub use std::io::Write;
    pub use std::str::FromStr;

    pub(super) use super::scheduler::prelude::*;

    pub use rumbo_logic::prelude::*;
}
use prelude::*;

mod config;
mod instances_controller;
mod metrics_controller;
mod scheduler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    build_logger();
    info!("Program started");

    let config = get_config();
    info!("Config is loaded");

    let mut scheduler = ActixJobScheduler::new();
    info!("Scheduler created");

    let app_sate = get_app_state(&config, &mut scheduler).await.unwrap();
    info!("App state created");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_sate.clone()))
            .wrap(middleware::Logger::default())
            .service(metrics_controller::get_metric)
            .service(metrics_controller::create_metric)
            .service(metrics_controller::delete_metric)
            .service(metrics_controller::update_metric)
            .service(instances_controller::get_instance)
            .service(instances_controller::create_instance)
            .service(instances_controller::delete_instance)
            .service(instances_controller::update_instance)
            .service(fs::Files::new("/", config.static_files_path).index_file("index.html"))
    })
    .bind((config.host_address, config.port))?
    .run()
    .await
}

async fn get_app_state<T>(config: &ConfigValues, job_scheduler: &mut T) -> Result<RumboApp>
where
    T: JobScheduler,
{
    let app = RumboApp::new(
        &config.db_url,
        job_scheduler
    )
    .await?;

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
        .filter(None, LevelFilter::Info)
        .filter(None, LevelFilter::Error)
        .filter(None, LevelFilter::Warn)
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
