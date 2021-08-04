use crate::*;

#[derive(Debug, Clone)]
pub struct Move(pub Pos, pub Axis, pub i8);

pub trait ReverseMoves {
  fn reverse_moves(self) -> Self;
}

impl ReverseMoves for Vec<Move> {
  fn reverse_moves(mut self) -> Self {
    self.reverse();
    for m in self.iter_mut() {
      m.2 = -m.2;
    }
    self
  }
}
