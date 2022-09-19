mod lobby_manager;

use actix::*;

pub use lobby_manager::{LobbyManager, LobbyId};
use crate::game::Game;
use crate::client_connection::ClientConnection;
use crate::api::message::LobbyMessage;

enum LobbyStatus {
  OnePlayerWaiting,
  TwoPlayersWaiting,
  GameStarted,
  GameFinished
}

pub struct Lobby {
  lobby_id: LobbyId,
  user1_connection: Option<Addr<ClientConnection>>,
  user2_connection: Option<Addr<ClientConnection>>,
  is_user1_black: bool,
  lobby_status: LobbyStatus,
  game: Option<Game>,
  lobby_manager: Addr<LobbyManager>
}

impl Lobby {
  pub fn new(lobby_id: LobbyId, user1_connection: Addr<ClientConnection>, is_user1_black: bool,
      lobby_manager: Addr<LobbyManager>) -> Lobby {
    // TODO: Create lobby id
    Lobby {
      lobby_id: lobby_id,
      user1_connection: Some(user1_connection),
      user2_connection: None,
      is_user1_black: is_user1_black,
      lobby_status: LobbyStatus::OnePlayerWaiting,
      game: None,
      lobby_manager: lobby_manager
    }
  }

  pub fn join(&mut self, user2_connection: Addr<ClientConnection>) {
    // TODO: check existing
    self.user2_connection = Some(user2_connection)
  }

  pub fn start_game(&mut self) {
    // TODO: check existing
    self.game = Some(Game::new())
  }
}

impl Actor for Lobby {
  type Context = Context<Self>;
}

impl Handler<LobbyMessage> for Lobby {
  type Result = ();

  fn handle(&mut self, msg: LobbyMessage, ctx: &mut Self::Context) -> Self::Result {
    // TODO
  }
}
