mod client_connection_manager;

use crate::api::message::{ClientMessage, ClientConnectionMessage};
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
    // TODO: handle client connection message (eg. sending back to client, etc.)
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
              // Send message to lobby manager
              // Update lobby address
            },
            ClientMessage::JoinLobby { lobby_id: lobby_id } => {
              // Send message to lobby manager
              // Update lobby address
            },
            ClientMessage::StartLobby => {
              // Send start message to lobby?
            },
            ClientMessage::PlayerMove { move_type: move_type } => {
              // Send move message to lobby
            },
            _ => ()
          }
        }
      },
      _ => ()
    }
  }
}
