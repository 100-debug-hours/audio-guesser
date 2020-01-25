mod config;
mod api;

use actix_files::Files;
use actix_web::{web, client::Client};

pub use config::*;

pub fn configure_actix_app(cfg: &mut web::ServiceConfig) {
    cfg
        .data(Client::default())
        .data(web::JsonConfig::default().limit(4096))
        .service(web::resource("/api/recognize_text").route(web::post().to(api::recognize_text)))
        .service(web::resource("/api/recognize_file").route(web::post().to(api::recognize_file)))
        .service(Files::new("/", "dist").index_file("index.html"));
}
