use actix::prelude::Addr;
use super::lobby_api::LobbyManager;
use crate::lobby::LobbyId;
use crate::client_connection::*;
use std::collections::HashMap;

pub struct ClientConnectionManager {
  lobby_manager: Addr<LobbyManager>;
  client_connections_map: HashMap<SessionToken, Addr<ClientConnection>>;
  lobby_ids_map: HashMap<SessionToken, LobbyId>;
}

impl ClientConnectionManager {
  pub fn new(lobby_manager_addr: Addr<LobbyManager>) {
    ClientConnectionManager {
      lobby_manager: lobby_manager_addr,
      client_connections_map: HashMap::new(),
      lobby_ids_map: HashMap::new()
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

impl Handler<ClientMessage> for ClientConnectionManager {
  type Result = ();

  fn handle(&mut self, msg: ClientMessage, ctx: &mut Self::Context) -> Self::Result {
    // TODO
  }
}
