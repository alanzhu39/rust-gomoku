use actix::*;

use std::collections::HashMap;
use uuid::Uuid;

use super::Lobby;
use crate::api::message::{ClientConnectionMessage, LobbyManagerMessage};
use crate::client_connection::ClientConnection;

pub type LobbyId = Uuid;

pub struct LobbyManager {
  lobbies_map: HashMap<LobbyId, Addr<Lobby>>
}

impl LobbyManager {
  pub fn new() -> LobbyManager {
    LobbyManager {
      lobbies_map: HashMap::new()
    }
  }
}

impl Actor for LobbyManager {
  type Context = Context<Self>;
}

impl Handler<LobbyManagerMessage> for LobbyManager {
  type Result = ();

  fn handle(&mut self, msg: LobbyManagerMessage, ctx: &mut Self::Context) -> Self::Result {
    match msg {
      LobbyManagerMessage::CreateLobby { user_connection: user1_connection } => {
        // Create lobby
        let lobby_id = Uuid::new_v4();
        let is_user1_black = true;
        let lobby_addr = Lobby::new(lobby_id, user1_connection.clone(), is_user1_black, ctx.address()).start();

        // Update map
        self.lobbies_map.insert(lobby_id, lobby_addr.clone());

        // Send message to client connection
        user1_connection.do_send(ClientConnectionMessage::LobbyJoined { lobby_addr: lobby_addr });
      },
      LobbyManagerMessage::JoinLobby { lobby_id: lobby_id, user_connection: user2_connection } => {
        // Send lobby join message
        let lobby_addr = self.lobbies_map.get(lobby_id).unwrap()
          .do_send(LobbyMessage::ClientJoinLobby { user_connection: user2_connection });
      },
      CloseLobby => {
        // TODO
      }
    }
  }
}
