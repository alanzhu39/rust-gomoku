mod messages;

pub use messages::*;
use actix::*;
use uuid::Uuid;
use actix_web_actors::ws;
use crate::api::ClientConnectionManager;

pub type SessionToken = Uuid;

pub struct ClientConnection {
  session_token: SessionToken,
  client_connection_manager: Addr<ClientConnectionManager>,
}

impl ClientConnection {
  fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
    // TODO: heartbeat
  }
}

impl Actor for ClientConnection {
  type Context = ws::WebsocketContext<Self>;
}

impl Handler<ServerMessage> for ClientConnection {
  type Result = ();

  fn handle(&mut self, msg: ServerMessage, ctx: &mut Self::Context) {
    // TODO: handle client connection manager message (sending back to client)
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