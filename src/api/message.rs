use actix::*;
use uuid::Uuid;

use crate::client_connection::{SessionToken, ClientConnection};
use crate::lobby::LobbyId;
use crate::game::MoveType;

pub enum ClientMessage {
  CreateLobby,
  JoinLobby { lobby_id: LobbyId },
  StartLobby,
  PlayerMove { move_type: MoveType },
  Rematch
}

impl ClientMessage {
  pub fn parse(text: String) -> Result<ClientMessage, ParsingError> {
    // TODO: parse text message
    let parts = text.split_once("::");
    match parts {
      Some((first, rest)) => {
        match first {
          "CREATE_LOBBY" => {
            Ok(ClientMessage::CreateLobby)
          },
          "JOIN_LOBBY" => {
            if let Ok(lobby_id) = Uuid::parse_str(rest) {
              Ok(ClientMessage::JoinLobby{ lobby_id: lobby_id })
            } else {
              Err(ParsingError)
            }
          },
          "START_LOBBY" => {
            Ok(ClientMessage::StartLobby)
          },
          "PLAYER_MOVE" => {
            if let Ok(move_type) = Self::parse_player_move(rest) {
              Ok(ClientMessage::PlayerMove{ move_type: move_type })
            } else {
              Err(ParsingError)
            }
          },
          _ => Err(ParsingError)
        }
      },
      _ => Err(ParsingError)
    }
  }

  fn parse_player_move(move_text: &str) -> Result<MoveType, ParsingError> {
    let mut coords = move_text.chars();
    let col = coords.next();
    let row = coords.next();

    // TODO: handle forfeit
    if let (Some(col), Some(row)) = (col, row) {
      Ok(MoveType::PlacePiece(col as usize - ('a' as usize), row as usize))
    } else {
      Err(ParsingError)
    }
  }
}

pub struct ParsingError;

pub enum ClientConnectionManagerMessage {
  AddClientConnection { session_token: SessionToken, client_connection_addr: Addr<ClientConnection> },
  CloseClientConnection
}

pub enum ClientConnectionMessage {
  LobbyJoined,
  LobbyGameMove,
  LobbyGameFinished
}

pub enum LobbyManagerMessage {
  CreateLobby,
  CloseLobby
}

pub enum LobbyMessage {
  ClientGameMove,
  ClientLeaveLobby,
  ClientJoinLobby,
  ClientRematch
}

impl Message for ClientConnectionManagerMessage {
  type Result = ();
}

impl Message for ClientConnectionMessage {
  type Result = ();
}

impl Message for LobbyManagerMessage {
  type Result = ();
}

impl Message for LobbyMessage {
  type Result = ();
}
