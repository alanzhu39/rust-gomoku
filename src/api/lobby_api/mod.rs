mod lobby_manager;

use lobby_manager;
use actix_web::{web, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.route("/create", web::post().to(create_lobby));
}

fn create_lobby() -> HttpResponse {
  HttpResponse::Ok
}
