mod board;

pub use board::PieceType;
use board::Board;

#[derive(Debug)]
pub enum MoveType {
  PlacePiece(usize, usize),
  Forfeit
}

pub enum GameState {
  Win(PieceType),
  Draw,
  InProgress
}

pub struct Game {
  board: Board,
  pub current_turn: PieceType,
  move_sequence: Vec<(PieceType, MoveType)>,
  pub game_state: GameState
}

impl Game {
  pub fn new() -> Game {
    Game {
      board: Board::new(),
      current_turn: PieceType::Black,
      move_sequence: Vec::new(),
      game_state: GameState::InProgress
    }
  }

  fn set_game_win(&mut self, piece_type: PieceType) {
    assert!(matches!(self.game_state, GameState::InProgress), "Game is already over!");

    self.game_state = GameState::Win(piece_type);
  }

  pub fn make_move(&mut self, move_type: MoveType) {
    assert!(matches!(self.game_state, GameState::InProgress), "Game is already over!");

    match move_type {
      MoveType::PlacePiece(x, y) => {
        // Place piece
        self.board.place_piece(x, y, self.current_turn);

        // Check win
        if self.board.has_five {
          self.set_game_win(self.current_turn);
        }

        // Update move_sequence
        self.move_sequence.push((self.current_turn.clone(), MoveType::PlacePiece(x, y)));

        // Update current turn
        self.current_turn = self.current_turn.other();
      },
      MoveType::Forfeit => self.set_game_win(self.current_turn.other())
    }
  }
}
