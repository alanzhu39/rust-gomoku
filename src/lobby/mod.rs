mod lobby_manager;

pub use lobby_manager::{LobbyManager, LobbyId};
use crate::game::Game;

struct OnePlayerWaitingLobby {
  user1_connection: String,
  is_user1_black: bool
}

impl OnePlayerWaitingLobby {
  pub fn new(user1_connection: String, is_user1_black: bool) -> OnePlayerWaitingLobby{
    OnePlayerWaitingLobby {
      user1_connection: user1_connection,
      is_user1_black: is_user1_black
    }
  }
  
  pub fn join_lobby(self, user2_connection: String) -> TwoPlayersWaitingLobby {
    TwoPlayersWaitingLobby::new(self, user2_connection)
  }
}

struct TwoPlayersWaitingLobby {
  user1_connection: String,
  user2_connection: String,
  is_user1_black: bool
}

impl TwoPlayersWaitingLobby {
  fn new(one_player_lobby: OnePlayerWaitingLobby, user2_connection: String) -> TwoPlayersWaitingLobby {
    TwoPlayersWaitingLobby {
      user1_connection: one_player_lobby.user1_connection,
      user2_connection: user2_connection,
      is_user1_black: one_player_lobby.is_user1_black
    }
  }

  pub fn set_is_user1_black(&mut self, is_user1_black: bool) {
    self.is_user1_black = is_user1_black;
  }

  pub fn start_lobby_game(self) -> GameStartedLobby {
    GameStartedLobby::new(self)
  }
}

struct GameStartedLobby {
  user1_connection: String,
  user2_connection: String,
  is_user1_black: bool,
  game: Game
}

impl GameStartedLobby {
  fn new(two_player_lobby: TwoPlayersWaitingLobby) -> GameStartedLobby {
    let player_black_id;
    let player_white_id;
    if two_player_lobby.is_user1_black {
      player_black_id = two_player_lobby.user1_connection.clone();
      player_white_id = two_player_lobby.user2_connection.clone();
    } else {
      player_black_id = two_player_lobby.user2_connection.clone();
      player_white_id = two_player_lobby.user1_connection.clone();
    }

    GameStartedLobby {
      user1_connection: two_player_lobby.user1_connection,
      user2_connection: two_player_lobby.user2_connection,
      is_user1_black: two_player_lobby.is_user1_black,
      game: Game::new()
    }
  }
}
