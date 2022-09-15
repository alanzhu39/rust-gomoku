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

impl Handler<ClientConnectionManagerMessage> for ClientConnectionManager {
  type Result = ();

  fn handle(&mut self, msg: ClientConnectionManagerMessage, ctx: &mut Self::Context) -> Self::Result {
    // TODO
  }
}
