// mod api;
// mod database;
// mod logging;
mod game;
mod lobby;
mod client_connection;
mod api;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use api::{config, ClientConnectionManager, LobbyManager};
use actix::*;

#[get("/")]
async fn hello() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
  HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
  HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let lobby_manager_addr = LobbyManager::new().start();
  let client_connection_manager_addr = ClientConnectionManager::new(lobby_manager_addr.clone()).start();

  HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(lobby_manager_addr.clone()))
      .app_data(web::Data::new(client_connection_manager_addr.clone()))
      .service(hello)
      .service(echo)
      .route("/hey", web::get().to(manual_hello))
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
