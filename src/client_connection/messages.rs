use actix::*;

pub enum ClientMessage {
  Todo
}

impl Message for ClientMessage {
  type Result = ();
}

impl ClientMessage {
  pub fn parse(text: &str) -> ClientMessage {
    // TODO: parse text message
    ClientMessage::Todo
  }
}

pub enum ServerMessage {

}

impl Message for ServerMessage {
  type Result = ();
}
