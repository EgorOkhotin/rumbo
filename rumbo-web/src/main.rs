use actix_files as fs;
use actix_web::{middleware, App, HttpServer};
use std::io::Write;
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();
    

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())

            // for deploy
            .service(fs::Files::new("/", "./static/").index_file("index.html"))

            // // for local running
            // .service(fs::Files::new("/", "./target/debug/static").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
