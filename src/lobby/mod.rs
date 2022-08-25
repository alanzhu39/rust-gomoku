use crate::game::Game;

pub struct OnePlayerWaitingLobby {
  user1_id: String,
  is_user1_black: bool
}

impl OnePlayerWaitingLobby {
  pub fn new(user1_id: String, is_user1_black: bool) -> OnePlayerWaitingLobby{
    OnePlayerWaitingLobby {
      user1_id: user1_id,
      is_user1_black: is_user1_black
    }
  }
  
  pub fn join_lobby(self, user2_id: String) -> TwoPlayersWaitingLobby {
    TwoPlayersWaitingLobby::new(self, user2_id)
  }
}

pub struct TwoPlayersWaitingLobby {
  user1_id: String,
  user2_id: String,
  is_user1_black: bool
}

impl TwoPlayersWaitingLobby {
  fn new(one_player_lobby: OnePlayerWaitingLobby, user2_id: String) -> TwoPlayersWaitingLobby {
    TwoPlayersWaitingLobby {
      user1_id: one_player_lobby.user1_id,
      user2_id: user2_id,
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

pub struct GameStartedLobby {
  user1_id: String,
  user2_id: String,
  is_user1_black: bool,
  game: Game
}

impl GameStartedLobby {
  fn new(two_player_lobby: TwoPlayersWaitingLobby) -> GameStartedLobby {
    let player_black_id;
    let player_white_id;
    if two_player_lobby.is_user1_black {
      player_black_id = two_player_lobby.user1_id.clone();
      player_white_id = two_player_lobby.user2_id.clone();
    } else {
      player_black_id = two_player_lobby.user2_id.clone();
      player_white_id = two_player_lobby.user1_id.clone();
    }

    GameStartedLobby {
      user1_id: two_player_lobby.user1_id,
      user2_id: two_player_lobby.user2_id,
      is_user1_black: two_player_lobby.is_user1_black,
      game: Game::new(player_black_id, player_white_id)
    }
  }
}
