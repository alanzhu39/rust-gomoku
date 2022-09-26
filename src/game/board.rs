#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum PieceType {
  Black,
  White
}

impl PieceType {
  pub fn other(&self) -> PieceType {
    match self {
      PieceType::Black => PieceType::White,
      PieceType::White => PieceType::Black
    }
  }
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

  fn get_grid_index(x: usize, y: usize) -> usize {
    x * Board::SIZE_LIMIT + y
  }

  pub fn place_piece(&mut self, x: usize, y: usize, piece_type: PieceType) {
    if x >= Board::SIZE_LIMIT || y >= Board::SIZE_LIMIT {
      eprintln!("Invalid piece coordinates!");
      return;
    }

    let piece_code: u8 = match piece_type {
        PieceType::Black => 1,
        PieceType::White => 2
      };
    
    let grid_index = Board::get_grid_index(x, y);
    if self.grid[grid_index] != 0 {
      eprintln!("Board is not empty at location ({}, {})", x, y);
    }

    self.grid[grid_index] = piece_code;

    // Update has_five
    let directions: [((isize, isize), (isize, isize)); 4] = [
      ((-1, 0), (1, 0)),
      ((0, -1), (0, 1)),
      ((-1, -1), (1, 1)),
      ((-1, 1), (1, -1)),
    ];
    let ix = x as isize;
    let iy = y as isize;

    for (back, front) in directions {
      let mut counter = 1;

      for i in 1..5 {
        let ix = x as isize + i*back.0;
        let iy = y as isize + i*back.1;
        if ix < 0 || ix >= Board::SIZE_LIMIT as isize
            || iy < 0 || iy >= Board::SIZE_LIMIT as isize {
          break;
        }

        let cur_index = Board::get_grid_index(ix as usize, iy as usize);
        if self.grid[cur_index] == piece_code {
          counter += 1;
        } else {
          break;
        }
      }

      for i in 1..5 {
        let ix = x as isize + i*front.0;
        let iy = y as isize + i*front.1;
        if ix < 0 || ix >= Board::SIZE_LIMIT as isize
            || iy < 0 || iy >= Board::SIZE_LIMIT as isize {
          break;
        }

        let cur_index = Board::get_grid_index(ix as usize, iy as usize);
        if self.grid[cur_index] == piece_code {
          counter += 1;
        } else {
          break;
        }
      }

      if counter >= 5 {
        self.has_five = true;
        break;
      }
    }
  }
}
