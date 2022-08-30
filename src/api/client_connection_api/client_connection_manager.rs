use actix::prelude::Addr;
use super::lobby_api::LobbyManager;
use crate::client_connection::*;

pub struct ClientConnectionManager {
  lobby_manager: Addr<LobbyManager>;
  // Session token <> client connections map
  // Session token <> lobby ids map
}

impl ClientConnectionManager {
  pub fn create_client_connection(&mut self, session_token: String) {
    // TODO
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
