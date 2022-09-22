use uuid::Uuid;
use std::fmt;

use crate::api::message::*;
use crate::game::{Game, GameState, MoveType, PieceType};

#[test]
fn client_message_parsing_test() {
  assert!(matches!(ClientMessage::parse(String::from("CREATE_LOBBY")), Ok(ClientMessage::CreateLobby)));

  let lobby_id = Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8").unwrap();
  assert!(matches!(
    ClientMessage::parse(String::from("JOIN_LOBBY::67e55044-10b1-426f-9247-bb680e5fe0c8")),
    Ok(ClientMessage::JoinLobby{ lobby_id: lobby_id })));

  assert!(matches!(ClientMessage::parse(String::from("START_LOBBY")), Ok(ClientMessage::StartLobby)));

  assert!(matches!(
    ClientMessage::parse(String::from("PLAYER_MOVE::PIECE:a3")),
    Ok(ClientMessage::PlayerMove{ move_type: MoveType::PlacePiece(0, 2)})));

  assert!(matches!(
    ClientMessage::parse(String::from("PLAYER_MOVE::PIECE:o15")),
    Ok(ClientMessage::PlayerMove{ move_type: MoveType::PlacePiece(14, 14)})));

  assert!(matches!(
    ClientMessage::parse(String::from("PLAYER_MOVE::FORFEIT")),
    Ok(ClientMessage::PlayerMove{ move_type: MoveType::Forfeit})));
}

#[test]
fn format_client_connection_message_test() {
  assert_eq!(ClientConnectionMessage::LobbyGameMove {
    piece_type: PieceType::Black,
    move_type: MoveType::PlacePiece(0, 0)
  }.to_string(), "GAME_MOVE::BLACK:a1");

  assert_eq!(ClientConnectionMessage::LobbyGameMove {
    piece_type: PieceType::Black,
    move_type: MoveType::PlacePiece(5, 10)
  }.to_string(), "GAME_MOVE::BLACK:f11");

  assert_eq!(ClientConnectionMessage::LobbyGameMove {
    piece_type: PieceType::White,
    move_type: MoveType::PlacePiece(11, 1)
  }.to_string(), "GAME_MOVE::WHITE:l2");

  assert_eq!(ClientConnectionMessage::LobbyGameFinished.to_string(), "GAME_FINISHED");
}

#[test]
fn game_win_test() {
  let mut game = Game::new();
  assert!(matches!(
    game.game_state,
    GameState::InProgress));

  game.make_move(MoveType::PlacePiece(0, 0)); // Black
  game.make_move(MoveType::PlacePiece(1, 1));
  game.make_move(MoveType::PlacePiece(0, 1)); // Black
  game.make_move(MoveType::PlacePiece(3, 3));
  game.make_move(MoveType::PlacePiece(0, 2)); // Black
  game.make_move(MoveType::PlacePiece(5, 5));
  game.make_move(MoveType::PlacePiece(0, 3)); // Black
  game.make_move(MoveType::PlacePiece(7, 7));
  game.make_move(MoveType::PlacePiece(0, 4)); // Black
  assert!(matches!(
    game.game_state,
    GameState::Win(PieceType::Black)));

  game = Game::new();
  game.make_move(MoveType::PlacePiece(9, 9)); // Black
  game.make_move(MoveType::PlacePiece(0, 0));
  game.make_move(MoveType::PlacePiece(8, 8)); // Black
  game.make_move(MoveType::PlacePiece(0, 3));
  game.make_move(MoveType::PlacePiece(11, 11)); // Black
  game.make_move(MoveType::PlacePiece(0, 5));
  game.make_move(MoveType::PlacePiece(12, 12)); // Black
  game.make_move(MoveType::PlacePiece(0, 7));
  game.make_move(MoveType::PlacePiece(10, 10)); // Black
  assert!(matches!(
    game.game_state,
    GameState::Win(PieceType::Black)));

  game = Game::new();
  game.make_move(MoveType::PlacePiece(0, 0));
  game.make_move(MoveType::PlacePiece(9, 10)); // White
  game.make_move(MoveType::PlacePiece(0, 3));
  game.make_move(MoveType::PlacePiece(7, 8)); // White
  game.make_move(MoveType::PlacePiece(0, 5));
  game.make_move(MoveType::PlacePiece(6, 7)); // White
  game.make_move(MoveType::PlacePiece(0, 7));
  game.make_move(MoveType::PlacePiece(5, 6)); // White
  game.make_move(MoveType::PlacePiece(0, 9));
  game.make_move(MoveType::PlacePiece(8, 9)); // White
  assert!(matches!(
    game.game_state,
    GameState::Win(PieceType::White)));
}
