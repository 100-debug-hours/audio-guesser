use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::process;
use env_logger::Env;

#[actix_rt::main]
async fn main() {
    env_logger::from_env(Env::default().default_filter_or("debug")).init();

    if cfg!(debug_assertions) {
        if let Err(err) = dotenv() {
            log::error!("Failed to load \".env\" file: {}", err);
            process::exit(exitcode::CONFIG);
        }
    }

    let config = backend::get_config();

    let server = HttpServer::new(|| App::new().configure(backend::configure_actix_app))
        .bind(format!("0.0.0.0:{}", config.port))
        .unwrap_or_else(|err| {
            log::error!("Failed to bind to port {}: {}", config.port, err);
            process::exit(exitcode::IOERR)
        });

    if let Err(err) = server.run().await {
        log::error!("Unrecoverable IO error occured: {}", err);
        process::exit(exitcode::IOERR);
    }
}
