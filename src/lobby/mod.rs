mod lobby_manager;

use actix::*;

pub use lobby_manager::{LobbyManager, LobbyId};
use crate::game::Game;

enum LobbyStatus {
  OnePlayerWaiting,
  TwoPlayersWaiting,
  GameStarted,
  GameFinished
}

pub struct Lobby {
  lobby_id: LobbyId,
  user1_connection: Addr<ClientConnection>,
  user2_connection: Addr<ClientConnection>,
  is_user1_black: bool,
  lobby_status: LobbyStatus,
  game: Game,
  lobby_manager: Addr<LobbyManager>
}

impl Lobby {
  pub fn new(user1_connection: Addr<ClientConnection>, is_user1_black: bool) -> Lobby {
    // TODO: Create lobby id
    Lobby {
      user1_connection: user1_connection,
      is_user1_black: is_user1_black
    }
  }

  pub fn join(&mut self, user2_connection: Addr<ClientConnection>) {
    self.user2_connection = user2_connection
  }

  pub fn start_game(&mut self) {
    self.game = Game::new()
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
