use actix::prelude::Addr;
use super::lobby_api::LobbyManager;

pub struct ClientConnectionManager {
  lobby_manager: Addr<LobbyManager>;
  // Session token <> client connections map
  // Session token <> lobby ids map
}
