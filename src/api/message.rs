use actix::*;

pub enum ClientMessage {
  Todo
}

impl ClientMessage {
  pub fn parse(text: &str) -> ClientMessage {
    // TODO: parse text message
    ClientMessage::Todo
  }
}

pub enum ClientConnectionManagerMessage {
  CreateClientConnection,
  CloseClientConnection
}

pub enum ClientConnectionMessage {
  LobbyJoined,
  LobbyGameMove,
  LobbyGameFinished
}

pub enum LobbyManagerMessage {
  CreateLobby,
  CloseLobby
}

pub enum LobbyMessage {
  ClientGameMove,
  ClientLeaveLobby,
  ClientJoinLobby,
  ClientRematch
}

impl Message for ClientConnectionManagerMessage {
  type Result = ();
}

impl Message for ClientConnectionMessage {
  type Result = ();
}

impl Message for LobbyManagerMessage {
  type Result = ();
}

impl Message for LobbyMessage {
  type Result = ();
}
