mod client_connection_manager;

use crate::api::message::{ClientMessage, ClientConnectionMessage, LobbyMessage, LobbyManagerMessage};
use crate::lobby::{Lobby, LobbyManager};
pub use client_connection_manager::{ClientConnectionManager, ClientConnectionId};

use actix::*;
use actix_web_actors::ws;

use uuid::Uuid;

pub type SessionToken = Uuid;

pub struct ClientConnection {
  session_token: SessionToken,
  client_connection_manager: Addr<ClientConnectionManager>,
  lobby_manager: Addr<LobbyManager>,
  lobby: Option<Addr<Lobby>>
}

impl ClientConnection {
  pub fn new(session_token: SessionToken, client_connection_manager: Addr<ClientConnectionManager>,
      lobby_manager: Addr<LobbyManager>) -> ClientConnection {
    ClientConnection {
      session_token: session_token,
      client_connection_manager: client_connection_manager,
      lobby_manager: lobby_manager,
      lobby: None
    }
  }

  fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
    // TODO: heartbeat
  }
}

impl Actor for ClientConnection {
  type Context = ws::WebsocketContext<Self>;
}

impl Handler<ClientConnectionMessage> for ClientConnection {
  type Result = ();

  fn handle(&mut self, msg: ClientConnectionMessage, ctx: &mut Self::Context) {
    match msg {
      ClientConnectionMessage::LobbyJoined { lobby_addr: lobby_addr } => {
        self.lobby = Some(lobby_addr);
        // TODO: send lobby state thru websocket
      },
      ClientConnectionMessage::LobbyGameMove => {
        // TODO: send thru websocket ctx
      },
      ClientConnectionMessage::LobbyGameFinished => {
        // TODO: send thru websocket ctx
      },
    }
  }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ClientConnection {
  fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
    println!("{:?}", msg);
    match msg {
      Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
      Ok(ws::Message::Text(text)) => {
        let client_message = ClientMessage::parse(String::from(text));
        if let Ok(client_message) = client_message {
          match client_message {
            ClientMessage::CreateLobby => {
              self.lobby_manager.do_send(LobbyManagerMessage::CreateLobby { user1_connection: ctx.address() });
            },
            ClientMessage::JoinLobby { lobby_id: lobby_id } => {
              self.lobby_manager.do_send(LobbyManagerMessage::JoinLobby {
                lobby_id: lobby_id,
                user2_connection: ctx.address()
              });
            },
            ClientMessage::StartLobby => {
              if let Some(lobby) = &self.lobby {
                lobby.do_send(LobbyMessage::ClientStartLobby);
              } else {
                // TODO: error handling
              }
            },
            ClientMessage::PlayerMove { move_type: move_type } => {
              if let Some(lobby) = &self.lobby {
                lobby.do_send(LobbyMessage::ClientGameMove { move_type: move_type });
              } else {
                // TODO: error handling
              }
            },
            _ => ()
          }
        }
      },
      _ => ()
    }
  }
}
