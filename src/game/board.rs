#[derive(Copy, Clone)]
pub enum PieceType {
  Black,
  White
}

pub struct Board {
  /// 15x15 grid, row major indexing
  /// 0 = empty, 1 = black, 2 = white
  grid: [u8; 225],
  pub has_five: bool
}

impl Board {
  const SIZE_LIMIT: usize = 15;

  pub fn new() -> Board {
    Board {
      grid: [0; 225],
      has_five: false
    }
  }

  /// Place
  pub fn place_piece(&mut self, x: usize, y: usize, piece_type: PieceType) {
    assert!(x < Board::SIZE_LIMIT && y < Board::SIZE_LIMIT, "Invalid piece coordinates!");

    let piece_code: u8 = match piece_type {
        PieceType::Black => 1,
        PieceType::White => 2
      };
    
    let grid_index = x * Board::SIZE_LIMIT + y;
    assert!(self.grid[grid_index] == 0, "Board is not empty at location ({}, {})", x, y);

    self.grid[grid_index] = piece_code;

    // TODO: update has_five
  }
}
