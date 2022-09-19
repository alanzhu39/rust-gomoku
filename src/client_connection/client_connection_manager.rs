use actix::*;
use actix::Addr;

use std::collections::HashMap;
use uuid::Uuid;

use crate::api::message::{ClientMessage, ClientConnectionManagerMessage};
use super::{ClientConnection, SessionToken};

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
}

impl Actor for ClientConnectionManager {
  type Context = Context<Self>;
}

impl Handler<ClientConnectionManagerMessage> for ClientConnectionManager {
  type Result = ();

  fn handle(&mut self, msg: ClientConnectionManagerMessage, ctx: &mut Self::Context) -> Self::Result {
    match msg {
      ClientConnectionManagerMessage::AddClientConnection{session_token, client_connection_addr} => {
        self.client_connections_map.insert(session_token, client_connection_addr);
      },
      _ => {
        // TODO
      }
    }
  }
}
