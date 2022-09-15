mod client_connection_manager;

use crate::api::message::{ClientMessage, ServerMessage};
pub use client_connection_manager::{ClientConnectionManager, ClientConnectionId};

use actix::*;
use actix_web_actors::ws;

use uuid::Uuid;

pub type SessionToken = Uuid;

struct ClientConnection {
  session_token: SessionToken,
  client_connection_manager: Addr<ClientConnectionManager>,
  lobby_manager: Addr<LobbyManager>,
  lobby: Addr<Lobby>
}

impl ClientConnection {
  fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
    // TODO: heartbeat
  }
}

impl Actor for ClientConnection {
  type Context = ws::WebsocketContext<Self>;
}

impl Handler<ClientConnectionMessage> for ClientConnection {
  type Result = ();

  fn handle(&mut self, msg: ServerMessage, ctx: &mut Self::Context) {
    // TODO: handle client connection message (eg. sending back to client, etc.)
  }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ClientConnection {
  fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
    match msg {
      Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
      Ok(ws::Message::Text(text)) => {
        let client_message = ClientMessage::parse(&text);
        match client_message {
          _ => {
            // TODO: handle text
          }
        }
      },
      _ => ()
    }
  }
}
