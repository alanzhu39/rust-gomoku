mod game;
mod lobby;
mod client_connection;
mod api;

#[cfg(test)]
mod tests;

use actix::*;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;

use client_connection::ClientConnectionManager;
use lobby::LobbyManager;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let lobby_manager_addr = LobbyManager::new().start();
  let client_connection_manager_addr = ClientConnectionManager::new().start();

  HttpServer::new(move || {
    App::new()
      .wrap(
        Cors::default()
          .allowed_origin("http://localhost:3000")
          .allowed_methods(vec!["GET", "POST", "DELETE"])
          .allow_any_header()
          .max_age(3600)
      )
      .app_data(web::Data::new(lobby_manager_addr.clone()))
      .app_data(web::Data::new(client_connection_manager_addr.clone()))
      .configure(api::config)
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
