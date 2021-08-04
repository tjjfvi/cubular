use lazy_static::lazy_static;

use crate::*;

pub struct RootCube {
  pub values: [[[Value; 9]; 9]; 9],
  pub moves: Vec<Move>,
}

lazy_static! {
pub static ref SOLVED: [[[Value; 9]; 9]; 9] = {
  #[inline(never)] // Prevent huge compiled sizes
  fn x9<F: Fn((usize, I)) -> T, I: Copy, T>(cb: F) -> impl Fn(I) -> [T; 9] {
    move |i| {
    [
    cb((0, i)),
    cb((1, i)),
    cb((2, i)),
    cb((3, i)),
    cb((4, i)),
    cb((5, i)),
    cb((6, i)),
    cb((7, i)),
    cb((8, i)),
    ]
    }
  }

  x9(x9(x9(|(x, (y, (z, _)))| Value((x + y + z) % 18))))(())
};
}

impl RootCube {
  #[must_use]
  pub fn new(values: [[[Value; 9]; 9]; 9]) -> RootCube {
    RootCube {
      values,
      moves: vec![],
    }
  }
  pub fn solved() -> RootCube {
    RootCube::new(*SOLVED)
  }
  pub fn reset_moves(&mut self) {
    self.moves.clear();
  }
}

impl Cube for RootCube {
  fn get(&self, pos: Pos) -> Value {
    self.values[pos.0][pos.1][pos.2]
  }
  fn get_solved(&self, pos: Pos) -> Value {
    SOLVED[pos.0][pos.1][pos.2]
  }
  fn apply_move(&mut self, mut m: Move) {
    m.2 = m.2.rem_euclid(4);
    if m.2 == 0 {
      return;
    }
    let corner = m.0 - Pos(1, 1, 1);
    let pairs: Vec<_> = self
      .slice(corner, Pos(3, 3, 3))
      .iter()
      .map(|(p, v)| (p.rotate(m.1, m.2, 3) + corner, v))
      .collect();
    for (pos, val) in pairs {
      self.values[pos.0][pos.1][pos.2] = val;
    }
    self.moves.push(m);
  }
  fn size(&self) -> Pos {
    Pos(9, 9, 9)
  }
}
