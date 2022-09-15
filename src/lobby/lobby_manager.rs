use actix::*;

use super::{OnePlayerWaitingLobby, TwoPlayersWaitingLobby, GameStartedLobby};

use uuid::Uuid;

pub type LobbyId = Uuid;

pub struct LobbyManager {
  // TODO:
  // Lobby maps
}

impl LobbyManager {
  pub fn new() -> LobbyManager {
    // TODO
    LobbyManager {}
  }

  pub fn create_lobby() {
    // TODO
  }
}

impl Actor for LobbyManager {
  type Context = Context<Self>;
}

impl Handler<LobbyManagerMessage> for LobbyManager {
  type Result = ();

  fn handle(&mut self, msg: LobbyManagerMessage, ctx: &mut Self::Context) -> Self::Result {
    // TODO
  }
}
