use actix::*;
use actix::Addr;

use std::collections::HashMap;
use uuid::Uuid;

use crate::api::message::{ClientMessage, ClientConnectionMessage, ClientConnectionManagerMessage, LobbyMessage};
use crate::lobby::Lobby;
use super::ClientConnection;

pub type SessionToken = Uuid;

pub struct ClientConnectionManager {
  client_connections_map: HashMap<SessionToken, Addr<ClientConnection>>,
  lobbies_map: HashMap<SessionToken, Addr<Lobby>>
}

impl ClientConnectionManager {
  pub fn new() -> ClientConnectionManager {
    ClientConnectionManager {
      client_connections_map: HashMap::new(),
      lobbies_map: HashMap::new()
    }
  }
}

impl Actor for ClientConnectionManager {
  type Context = Context<Self>;
}

impl Handler<ClientConnectionManagerMessage> for ClientConnectionManager {
  type Result = ();

  fn handle(&mut self, msg: ClientConnectionManagerMessage, ctx: &mut Self::Context) -> Self::Result {
    match msg {
      ClientConnectionManagerMessage::AddClientConnection { user_token, client_connection_addr } => {
        // Update client connections map
        let session_token = if self.client_connections_map.contains_key(&user_token) {
          user_token
        } else {
          Uuid::new_v4()
        };

        if let Some(old_connection_addr)
            = self.client_connections_map.insert(session_token, client_connection_addr.clone()) {
          // Update new connection
          if let Some(lobby) = self.lobbies_map.get(&session_token) {
            lobby.do_send(LobbyMessage::ClientUpdateConnection {
              old_connection: old_connection_addr,
              new_connection: client_connection_addr.clone()
            });
          }
        }

        // Send session token message
        client_connection_addr.do_send(ClientConnectionMessage::SessionToken { session_token: session_token });
      },

      ClientConnectionManagerMessage::LobbyClientConnection { session_token, lobby_addr } => {
        if let Some(lobby_addr) = lobby_addr {
          self.lobbies_map.insert(session_token, lobby_addr);
        }
      }

      _ => {
        // TODO
      }
    }
  }
}
