mod client_connection_manager;

use crate::api::message::{ClientMessage, ClientConnectionMessage, LobbyMessage, LobbyManagerMessage};
use crate::lobby::{Lobby, LobbyManager, LobbyStatus};
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
  pub fn new(
    session_token: SessionToken,
    client_connection_manager: Addr<ClientConnectionManager>,
    lobby_manager: Addr<LobbyManager>
  ) -> ClientConnection {
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
    let message_text = msg.to_string();

    match msg {
      ClientConnectionMessage::LobbyStatus { lobby_addr, lobby_status, .. } => {
        match lobby_status {
          LobbyStatus::Closed => {
            self.lobby = None;
          }
          _ => {
            self.lobby = Some(lobby_addr);
          }
        }
      },
      _ => ()
    }

    ctx.text(message_text);
  }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ClientConnection {
  fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
    match msg {
      Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
      Ok(ws::Message::Close(_)) => println!("Closed: {:?}", ctx.address()),
      Ok(ws::Message::Text(text)) => {
        println!("Message: {}", text);

        if let Ok(client_message) = ClientMessage::parse(String::from(text)) {
          match client_message {
            ClientMessage::CreateLobby => {
              if let None = self.lobby {
                self.lobby_manager.do_send(LobbyManagerMessage::CreateLobby { user_connection: ctx.address() });
              }
            },
            
            ClientMessage::JoinLobby { lobby_id } => {
              if let None = self.lobby {
                self.lobby_manager.do_send(LobbyManagerMessage::JoinLobby {
                  lobby_id: lobby_id,
                  user_connection: ctx.address()
                });
              }
            },

            ClientMessage::StartLobby => {
              if let Some(lobby) = &self.lobby {
                lobby.do_send(LobbyMessage::ClientStartLobby { user_connection: ctx.address() });
              }
            },

            ClientMessage::LeaveLobby => {
              if let Some(lobby) = &self.lobby {
                lobby.do_send(LobbyMessage::ClientLeaveLobby { user_connection: ctx.address() });
              }
            },

            ClientMessage::Rematch => {
              if let Some(lobby) = &self.lobby {
                lobby.do_send(LobbyMessage::ClientRematch);
              }
            },

            ClientMessage::PlayerMove { move_type } => {
              if let Some(lobby) = &self.lobby {
                lobby.do_send(LobbyMessage::ClientGameMove { move_type: move_type, user_connection: ctx.address() });
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
