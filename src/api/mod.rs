pub mod lobby_api;
pub mod client_connection_api;

use actix_web::web;
pub use lobby_api::lobby_manager::LobbyManager;
pub use client_connection_api::client_connection_manager::ClientConnectionManager;

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::resource("/")
      .route(web::get().to(HttpResponse::Ok))
      .route(web::head().to(HttpResponse::MethodNotAllowed)),
  )
  .service(web::scope("/lobby").configure(lobby_api::config));
}
