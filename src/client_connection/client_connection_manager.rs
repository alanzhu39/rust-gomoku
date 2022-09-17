use actix::*;
use actix::Addr;

use crate::lobby::{LobbyManager, LobbyId};
use crate::api::message::ClientMessage;
use super::{ClientConnection, SessionToken};

use std::collections::HashMap;
use uuid::Uuid;

pub type ClientConnectionId = Uuid;

pub struct ClientConnectionManager {
  client_connections_map: HashMap<SessionToken, Addr<ClientConnection>>
}

impl ClientConnectionManager {
  pub fn new() -> ClientConnectionManager {
    ClientConnectionManager {
      client_connections_map: HashMap::new()
    }
  }

  pub fn create_client_connection(&mut self) {
    // TODO
    // Generate UUID
    // Start websocket
    // Update map
  }
}

impl Actor for ClientConnectionManager {
  type Context = Context<Self>;
}

impl Handler<ClientConnectionManagerMessage::CreateClientConnection> for ClientConnectionManager {
  type Result = Result<HttpResponse, Error>;

  fn handle(&mut self, msg: ClientConnectionManagerMessage::CreateClientConnection,
      ctx: &mut Self::Context) -> Self::Result {

    let client_connection = ClientConnection::new(ctx.address(), lobby_manager);

    let resp = ws::start_with_addr(ClientConnection::new(), &req, stream);

    match resp {
      Ok(client_connection_address, http_response) => {
        client_connections_map.insert(client_connection.session_token.clone(), client_connection_address);
        Ok(http_response)
      },
      Error(e) => Error(e),
    }
  }
}

impl Handler<ClientConnectionManagerMessage::CloseClientConnection> for ClientConnectionManager {
  type Result = ();

  fn handle(&mut self, msg: ClientConnectionManagerMessage::CloseClientConnection, ctx: &mut Self::Context) -> Self::Result {
    // TODO
  }
}
