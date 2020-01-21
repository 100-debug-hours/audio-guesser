mod config;

use actix_files::Files;
use actix_web::web;

pub use config::*;

pub fn configure_actix_app(cfg: &mut web::ServiceConfig) {
    // FIXME: change static files path to frontend bundle directory when frontend is bootstrapped
    cfg
        .service(Files::new("/static", "static").index_file("index.html"));
}
