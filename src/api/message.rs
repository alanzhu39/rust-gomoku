use actix::*;
use uuid::Uuid;
use std::fmt;

use crate::client_connection::{SessionToken, ClientConnection};
use crate::lobby::{Lobby, LobbyId, LobbyStatus};
use crate::game::{PieceType, MoveType};

#[derive(Debug)]
pub enum ClientMessage {
  CreateLobby,
  JoinLobby { lobby_id: LobbyId },
  StartLobby,
  LeaveLobby,
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
          Ok(ClientMessage::JoinLobby { lobby_id: lobby_id })
        } else {
          Err(ParsingError)
        }
      },
      "START_LOBBY" => {
        Ok(ClientMessage::StartLobby)
      },
      "LEAVE_LOBBY" => {
        Ok(ClientMessage::LeaveLobby)
      },
      "REMATCH" => {
        Ok(ClientMessage::Rematch)
      },
      "PLAYER_MOVE" => {
        if let Ok(move_type) = Self::parse_player_move(args) {
          Ok(ClientMessage::PlayerMove { move_type: move_type })
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
      "RESIGN" => {
        Ok(MoveType::Resign)
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
  LobbyStatus { lobby_id: LobbyId, lobby_status: LobbyStatus, lobby_addr: Addr<Lobby> },
  LobbyGameMove { piece_type: PieceType, move_type: MoveType }
}

impl fmt::Display for ClientConnectionMessage {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      ClientConnectionMessage::LobbyStatus { lobby_id, lobby_status, .. } => {
        write!(f, "LOBBY_STATUS::{}:{:?}", lobby_id.simple().encode_lower(&mut Uuid::encode_buffer()), lobby_status)
      },

      ClientConnectionMessage::LobbyGameMove { piece_type, move_type } => {
        let piece_str = if let PieceType::Black = piece_type { "BLACK" } else { "WHITE" };

        match move_type {
          MoveType::PlacePiece(x, y) => {
            let row = char::from_u32((x + ('a' as usize)).try_into().unwrap()).unwrap();
            let col = y + 1;

            write!(f, "GAME_MOVE::{}:{}{}", piece_str, row, col)
          },
          MoveType::Resign => {
            write!(f, "GAME_MOVE::{}:RESIGN", piece_str)
          }
        }
      },
    }
  }
}

pub enum LobbyManagerMessage {
  CreateLobby { user_connection: Addr<ClientConnection> },
  JoinLobby { lobby_id: LobbyId, user_connection: Addr<ClientConnection> },
  CloseLobby { lobby_id: LobbyId }
}

pub enum LobbyMessage {
  ClientJoinLobby { user_connection: Addr<ClientConnection> },
  ClientStartLobby { user_connection: Addr<ClientConnection> },
  ClientGameMove { move_type: MoveType, user_connection: Addr<ClientConnection> },
  ClientLeaveLobby { user_connection: Addr<ClientConnection> },
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
