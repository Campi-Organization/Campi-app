#[macro_use]
extern crate diesel;

mod schema;
mod db;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // Connecting to db

    // Hosting server
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            // .app_data()
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
