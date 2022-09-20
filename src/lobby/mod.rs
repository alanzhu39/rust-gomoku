mod lobby_manager;

use actix::*;

pub use lobby_manager::{LobbyManager, LobbyId};
use crate::game::Game;
use crate::client_connection::ClientConnection;
use crate::api::message::{ClientConnectionMessage, LobbyMessage};

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
}

impl Actor for Lobby {
  type Context = Context<Self>;
}

impl Handler<LobbyMessage> for Lobby {
  type Result = ();

  fn handle(&mut self, msg: LobbyMessage, ctx: &mut Self::Context) -> Self::Result {
    match msg {
      LobbyMessage::ClientJoinLobby { user_connection: user2_connection } => {
        // TODO: check existing
        self.user2_connection = Some(user2_connection.clone());
        self.lobby_status = LobbyStatus::TwoPlayersWaiting;
        user2_connection.do_send(ClientConnectionMessage::LobbyJoined { lobby_addr: ctx.address() });
      },
      LobbyMessage::ClientStartLobby { user_connection: user_connection } => {
        // TODO: check existing
        self.game = Some(Game::new());
        self.lobby_status = LobbyStatus::GameStarted;
      },
      LobbyMessage::ClientGameMove { move_type: move_type, user_connection: user_connection } => {
        // TODO: verify lobby status
        // Verify turn
        let (current_user, other_user) = (is_user1_black && self.game.current_turn == PieceType::Black)
            || (!is_user1_black && self.game.current_turn == PieceType::White)
          ? (&self.user1_connection, &self.user2_connection) : (&self.user2_connection, &self.user1_connection);
        if current_user.unwrap() != user_connection {
          // TODO: error message
          return;
        }

        // Call game method
        let piece_type = self.game.current_turn.clone();
        self.game.make_move(move_type.clone());
        // TODO: update lobby status

        // Send client connection message
        // TODO: check lobby status
        other_user.unwrap().do_send(ClientConnectionMessage::LobbyGameMove {
          piece_type: piece_type,
          move_type: move_type.clone()
        });
      },
      LobbyMessage::ClientLeaveLobby { user_connection: user_connection } => {
        // TODO
      },
      LobbyMessage::ClientRematch => {
        // TODO
      }
    }
  }
}
