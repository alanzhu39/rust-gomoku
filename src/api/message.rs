use actix::*;
use uuid::Uuid;

use crate::client_connection::{SessionToken, ClientConnection};
use crate::lobby::LobbyId;
use crate::game::MoveType;

#[derive(Debug)]
pub enum ClientMessage {
  CreateLobby,
  JoinLobby { lobby_id: LobbyId },
  StartLobby,
  PlayerMove { move_type: MoveType },
  Rematch
}

impl ClientMessage {
  pub fn parse(text: String) -> Result<ClientMessage, ParsingError> {
    let mut command = text.as_str();
    let mut args = "";

    let parts = text.split_once("::");
    if let Some((first, rest)) = parts {
      command = first;
      args = rest;
    }

    match command {
      "CREATE_LOBBY" => {
        Ok(ClientMessage::CreateLobby)
      },
      "JOIN_LOBBY" => {
        if let Ok(lobby_id) = Uuid::parse_str(args) {
          Ok(ClientMessage::JoinLobby{ lobby_id: lobby_id })
        } else {
          Err(ParsingError)
        }
      },
      "START_LOBBY" => {
        Ok(ClientMessage::StartLobby)
      },
      "PLAYER_MOVE" => {
        if let Ok(move_type) = Self::parse_player_move(args) {
          Ok(ClientMessage::PlayerMove{ move_type: move_type })
        } else {
          Err(ParsingError)
        }
      },
      _ => Err(ParsingError)
    }
  }

  fn parse_player_move(move_text: &str) -> Result<MoveType, ParsingError> {
    let mut command = move_text;
    let mut args = "";

    let parts = move_text.split_once(":");
    if let Some((first, rest)) = parts {
      command = first;
      args = rest;
    }

    match command {
      "PIECE" => {
        let col = args.chars().nth(0);
        let row = &args[1..].parse::<usize>();

        if let (Some(col), Ok(row)) = (col, row) {
          Ok(MoveType::PlacePiece((col as usize) - ('a' as usize), *row - 1))
        } else {
          Err(ParsingError)
        }
      },
      "FORFEIT" => {
        Ok(MoveType::Forfeit)
      },
      _ => Err(ParsingError)
    }
  }
}

#[derive(Debug)]
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
