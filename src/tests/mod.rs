use uuid::Uuid;

use crate::api::message::*;
use crate::game::MoveType;

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
