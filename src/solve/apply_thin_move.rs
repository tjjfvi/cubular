use super::*;

pub trait ApplyThinMove: Cube + Sized {
  fn apply_thin_move(&mut self, Move(center, axis, amount): Move) {
    let mut offcenter = center;
    offcenter[axis] -= 1;
    let offaxis = if axis == Axis::X { Axis::Y } else { Axis::X };
    self.apply_move(Move(offcenter, axis, amount));
    self.apply_move(Move(center, offaxis, 2));
    self.apply_move(Move(offcenter, axis, -amount));
    self.apply_move(Move(center, offaxis, 2));
    self.apply_move(Move(center, axis, -amount));
  }
  fn apply_thin_moves(&mut self, moves: Vec<Move>) {
    for m in moves {
      self.apply_thin_move(m);
    }
  }
}

impl<T: Cube> ApplyThinMove for T {}
