mod db;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // Creating tables and syncing db
    let client = Data::new(db::new_client().await.unwrap());
    #[cfg(debug_assertions)]
    client._db_push().await.unwrap();

    // Hosting server
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(client.clone())
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
