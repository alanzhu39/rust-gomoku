mod lobby_manager

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
      web::resource("/")
        .route(web::get().to(HttpResponse::Ok))
        .route(web::head().to(HttpResponse::MethodNotAllowed)),
  )
  .service(web::scope("/lobby").configure(lobby_manager::config));
}