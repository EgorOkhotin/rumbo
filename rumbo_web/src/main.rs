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

    pub const DEFAULT_PAGE_SIZE: i64 = 200;
}
use prelude::*;

mod config;
mod instances_controller;
mod metrics_controller;
mod scheduler;

use rumbo_logic::{
    send_telegram_message,
    EmailAddress, EmailContent, SmtpCredential, send_smtp_email
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    info!("Load environment variables from .env file.");

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
            .service(metrics_controller::get_all_metrics)
            .service(metrics_controller::create_metric)
            .service(metrics_controller::delete_metric)
            .service(metrics_controller::update_metric)
            .service(instances_controller::get_instance)
            .service(instances_controller::get_all_instances)
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
    let app = RumboApp::new(&config.db_url, job_scheduler).await?;

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


pub fn send_email_notification(subject: &str, message: String)-> Result<()>{
    let mut reply_to = None;
    let username = env::var("SMTP_USERNAME")?;
    let password = env::var("SMTP_PASSWORD")?;
    let from = env::var("SMTP_FROM_EMAIL_ADDRESS")?;
    let to = env::var("SMTP_TO_EMAIL_ADDRESS")?;
    let reply_email = 
        env::var("SMTP_REPLY_TO_EMAIL_ADDRESS").unwrap_or_default();
    if !reply_email.is_empty() {
        reply_to = Some(reply_email.as_str());
    }
    
    let email_address = EmailAddress::new(&to, &from, reply_to);
    let credential: SmtpCredential = (username, password).into();
    let email_content: EmailContent = (subject, message).into();
    send_smtp_email(credential, email_address, email_content)?;
    Ok(())
}

pub fn send_telegram_notification(message: String)->Result<()>{
    let token = env::var("TELEGRAM_BOT_TOKEN")
    .map_err(|_|RumboError::GenericError("TELEGRAM_BOT_TOKEN not set".into()))?;
    let chat_id: i64 = env::var("TELEGRAM_CHAT_ID").map_err(
        |_|RumboError::GenericError("Missing TELEGRAM_CHAT_ID environment variable".into())
        )?.parse().map_err(
            |_|RumboError::GenericError("Error parsing TELEGRAM_CHAT_ID as i64".into())
        )?;
    send_telegram_message(message, &token, chat_id)?;
    Ok(())
}
