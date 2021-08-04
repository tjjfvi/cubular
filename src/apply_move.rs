use crate::*;

// pub trait ApplyMove: Cube + Sized {
//   fn apply_move(&mut self, Move(center, axis, amount): Move) {
//     if amount == 0 {
//       return;
//     }
//     let mut slice = self.slice(center - Pos(1, 1, 1), Pos(3, 3, 3));
//     let mapped: Vec<_> = (&mut slice).rotate(axis, amount).iter().collect();
//     for (pos, val) in mapped {
//       unsafe { slice.set(pos, val) }
//     }
//   }
//   fn apply_moves(&mut self, moves: Vec<Move>) {
//     for m in moves {
//       self.apply_move(m);
//     }
//   }
// }

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

// impl<T: Cube> ApplyMove for T {}
