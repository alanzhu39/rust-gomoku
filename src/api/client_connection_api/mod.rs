use actix::*;
use actix_web_actors::ws;
use actix_web::{web, HttpRequest, HttpResponse, Error};

use uuid::Uuid;

use crate::client_connection::{ClientConnection, ClientConnectionManager};
use crate::lobby::LobbyManager;
use crate::api::message::ClientConnectionManagerMessage;

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.route("", web::get().to(create_client_connection));
}

async fn create_client_connection(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
  let client_connection_manager = req.app_data::<web::Data<Addr<ClientConnectionManager>>>().unwrap().get_ref().clone();
  let lobby_manager = req.app_data::<web::Data<Addr<LobbyManager>>>().unwrap().get_ref().clone();

  // Generate session token
  let session_token = Uuid::new_v4();
  let client_connection = ClientConnection::new(session_token.clone(), client_connection_manager.clone(), lobby_manager);

  // Start websocket
  let resp = ws::WsResponseBuilder::new(client_connection, &req, stream).start_with_addr();

  // Update map
  match resp {
    Ok((client_connection_addr, http_response)) => {
      // TODO: error case on send
      client_connection_manager.do_send(ClientConnectionManagerMessage::AddClientConnection {
        session_token: session_token,
        client_connection_addr: client_connection_addr
      });
      // TODO: send session token
      Ok(http_response)
    },
    Err(e) => Err(e),
  }
}
