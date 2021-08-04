use crate::*;

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
