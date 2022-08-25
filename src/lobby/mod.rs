use crate::game::Game;

pub struct OnePlayerWaitingLobby {
  user1_id: String
}

impl OnePlayerWaitingLobby {
  pub fn new(user1_id: String) -> OnePlayerWaitingLobby{
    OnePlayerWaitingLobby {
      user1_id: user1_id
    }
  }
  
  pub fn join_lobby(&self, user2_id: String) -> TwoPlayersWaitingLobby {
    TwoPlayersWaitingLobby::new(self, user2_id)
  }
}

pub struct TwoPlayersWaitingLobby {
  user1_id: String,
  user2_id: String,
  is_user1_black: bool,
}

impl TwoPlayersWaitingLobby {
  fn new(one_player_lobby: OnePlayerWaitingLobby, user2_id: String) -> TwoPlayersWaitingLobby {
    TwoPlayersWaitingLobby {
      user2_id: user2_id
      ..one_player_lobby
    }
  }

  pub fn set_is_user1_black(&mut self, is_user1_black: bool) {
    self.is_user1_black = is_user1_black;
  }

  pub fn start_lobby_game(&self) -> GameStartedLobby {
    
  }
}

pub struct GameStartedLobby {
  user1_id: String,
  user2_id: String,
  is_user1_black: bool,
  game: Game
}

impl GameStartedLobby {
  fn new(two_player_lobby: TwoPlayersWaitingLobby) {
    let player_black_id = if is_user1_black { user1_id.clone() } else { user2_id.clone() };
    let player_white_id = if !is_user1_black { user1_id.clone() } else { user2_id.clone() };
    GameStartedLobby {
      game: Game::new(player_black_id, player_white_id),
      ..two_player_lobby
    }
  }
}
