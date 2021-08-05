use crate::*;
use lazy_static::lazy_static;

pub trait Cube {
  fn get(&self, pos: Pos) -> Value;
  fn get_solved(&self, pos: Pos) -> Value;
  fn apply_move(&mut self, m: Move);
  fn size(&self) -> Pos;
  fn apply_moves(&mut self, moves: Vec<Move>) {
    for m in moves {
      self.apply_move(m);
    }
  }
}

impl<T: Cube + ?Sized> Cube for &mut T {
  fn get(&self, pos: Pos) -> Value {
    (**self).get(pos)
  }
  fn get_solved(&self, pos: Pos) -> Value {
    (**self).get_solved(pos)
  }
  fn apply_move(&mut self, m: Move) {
    (**self).apply_move(m)
  }
  fn size(&self) -> Pos {
    (**self).size()
  }
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

impl Cube for [[[Value; 9]; 9]; 9] {
  fn get(&self, pos: Pos) -> Value {
    self[pos.0][pos.1][pos.2]
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
      self[pos.0][pos.1][pos.2] = val;
    }
  }
  fn size(&self) -> Pos {
    Pos(9, 9, 9)
  }
}
