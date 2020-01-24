mod config;

use actix_files::Files;
use actix_web::web;

pub use config::*;

pub fn configure_actix_app(cfg: &mut web::ServiceConfig) {
    cfg
        .service(Files::new("/assets", "assets"))
        .service(Files::new("/", "dist").index_file("index.html"));
}
