pub mod message;
mod lobby_api;
mod client_connection_api;

use actix_web::{web, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::resource("/")
      .route(web::get().to(HttpResponse::Ok))
      .route(web::head().to(HttpResponse::MethodNotAllowed)),
  )
  .service(web::scope("/lobby").configure(lobby_api::config));
}
