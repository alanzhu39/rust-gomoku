mod lobby_manager;

use actix::*;

pub use lobby_manager::{LobbyManager, LobbyId};
use crate::game::{PieceType, GameState, Game};
use crate::client_connection::ClientConnection;
use crate::api::message::{ClientConnectionMessage, LobbyMessage, LobbyManagerMessage};

#[derive(Copy, Clone, Debug)]
pub enum LobbyStatus {
  OnePlayerWaiting,
  TwoPlayersWaiting,
  GameStarted,
  GameFinished,
  Closed
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
  pub fn new(
    lobby_id: LobbyId,
    user1_connection: Addr<ClientConnection>,
    is_user1_black: bool,
    lobby_manager: Addr<LobbyManager>
  ) -> Lobby {
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

  pub fn new_empty(lobby_id: LobbyId, lobby_manager: Addr<LobbyManager>) -> Lobby {
    Lobby {
      lobby_id: lobby_id,
      user1_connection: None,
      user2_connection: None,
      is_user1_black: true,
      lobby_status: LobbyStatus::OnePlayerWaiting,
      game: None,
      lobby_manager: lobby_manager
    }
  }

  fn send_status_messages(&self, lobby_addr: Addr<Lobby>) {
    // Send lobby status messsages
    self.user1_connection.as_ref().unwrap().do_send(ClientConnectionMessage::LobbyStatus {
      lobby_id: self.lobby_id.clone(),
      lobby_status: self.lobby_status,
      lobby_addr: lobby_addr.clone()
    });
    self.user2_connection.as_ref().unwrap().do_send(ClientConnectionMessage::LobbyStatus {
      lobby_id: self.lobby_id.clone(),
      lobby_status: self.lobby_status,
      lobby_addr: lobby_addr.clone()
    });
  }
}

impl Actor for Lobby {
  type Context = Context<Self>;
}

impl Handler<LobbyMessage> for Lobby {
  type Result = ();

  fn handle(&mut self, msg: LobbyMessage, ctx: &mut Self::Context) -> Self::Result {
    match msg {
      LobbyMessage::ClientUpdateConnection { old_connection, new_connection } => {
        // Update user connection field
        if &old_connection == self.user1_connection.as_ref().unwrap() {
          self.user1_connection = Some(new_connection);
        } else if &old_connection == self.user2_connection.as_ref().unwrap() {
          self.user2_connection = Some(new_connection);
        } else {
          return;
        }

        // Send lobby status message
        self.send_status_messages(ctx.address());
      }

      LobbyMessage::ClientJoinLobby { user_connection: user2_connection } => {
        // check lobby status
        if !matches!(self.lobby_status, LobbyStatus::OnePlayerWaiting) {
          eprintln!("Cannot join lobby with status {:?}!", self.lobby_status);
          return;
        }
        
        // Update lobby
        self.user2_connection = Some(user2_connection.clone());
        self.lobby_status = LobbyStatus::TwoPlayersWaiting;

        // Send lobby joined messages
        self.send_status_messages(ctx.address());
      },

      LobbyMessage::ClientStartLobby { user_connection } => {
        // check lobby status
        if !matches!(self.lobby_status, LobbyStatus::TwoPlayersWaiting) {
          eprintln!("Cannot start lobby with status {:?}!", self.lobby_status);
          return;
        }

        // Check user connection
        if &user_connection != self.user1_connection.as_ref().unwrap() {
          eprintln!("Must be lobby creator to start lobby!");
          return;
        }

        // Start game
        self.game = Some(Game::new());
        self.lobby_status = LobbyStatus::GameStarted;

        // Send lobby started messsages
        self.send_status_messages(ctx.address());
      },

      LobbyMessage::ClientGameMove { move_type, user_connection } => {
        // verify lobby status
        if !matches!(self.lobby_status, LobbyStatus::GameStarted) {
          eprintln!("Cannot make game moves in lobby with status {:?}!", self.lobby_status);
          return;
        }

        // Get user piece type
        let game = self.game.as_mut().unwrap();
        let is_user1 = user_connection.eq(self.user1_connection.as_ref().unwrap());
        let user_piece_type =
          if (self.is_user1_black && is_user1) || (!self.is_user1_black && !is_user1) {
            PieceType::Black
          } else {
            PieceType::White
          };

        // Call game method
        if let Err(_) = game.make_move(user_piece_type, move_type.clone()) { return };

        // Send game move messages
        self.user1_connection.as_ref().unwrap().do_send(ClientConnectionMessage::LobbyGameMove {
          piece_type: user_piece_type,
          move_type: move_type.clone()
        });

        self.user2_connection.as_ref().unwrap().do_send(ClientConnectionMessage::LobbyGameMove {
          piece_type: user_piece_type,
          move_type: move_type.clone()
        });

        // Update lobby status
        match game.game_state {
          GameState::Win(_) | GameState::Draw => {
            self.lobby_status = LobbyStatus::GameFinished;

            // Send lobby status messages
            self.send_status_messages(ctx.address());
          },
          _ => ()
        }
      },

      LobbyMessage::ClientLeaveLobby { user_connection } => {
        // check lobby status
        if !matches!(self.lobby_status, LobbyStatus::GameFinished) {
          eprintln!("Cannot start lobby with status {:?}!", self.lobby_status);
          return;
        }

        // Start game
        self.lobby_status = LobbyStatus::Closed;

        // Send lobby status messsages
        self.send_status_messages(ctx.address());

        // Send lobby manager close message
        self.lobby_manager.do_send(LobbyManagerMessage::CloseLobby {
          lobby_id: self.lobby_id.clone()
        });

        ctx.stop();
      },

      LobbyMessage::ClientRematch => {
        // check lobby status
        if !matches!(self.lobby_status, LobbyStatus::GameFinished) {
          eprintln!("Cannot restart lobby with status {:?}!", self.lobby_status);
          return;
        }

        // Start game
        self.game = Some(Game::new());
        self.is_user1_black = !self.is_user1_black;
        self.lobby_status = LobbyStatus::GameStarted;

        // Send lobby started messsages
        self.send_status_messages(ctx.address());
      }
    }
  }
}
