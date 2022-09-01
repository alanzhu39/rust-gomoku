use crate::lobby;
use actix::*;
use uuid::Uuid;

pub type LobbyId = Uuid;

pub struct LobbyManager {
  // TODO:
  // Client connection manager address
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

  pub fn join_lobby() {
    // TODO
  }
}

impl Actor for LobbyManager {
  type Context = Context<Self>;
}
