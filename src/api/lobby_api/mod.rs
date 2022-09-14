pub mod lobby_manager;

use actix_web::{web, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.route("/create", web::get().to(create_lobby));
}

async fn create_lobby() -> HttpResponse {
  HttpResponse::Ok().json("test")
}
