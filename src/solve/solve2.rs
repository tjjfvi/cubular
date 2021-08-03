use crate::*;
use lazy_static::lazy_static;

pub trait Solve2: Cube + Sized {
  fn solve2(&self) {
    self.print();
    self._apply_thin_move(Move(Pos(4, 4, 4), Axis::Z, 1))
  }
}

lazy_static! {
  static ref INNER_POSS: Vec<Pos> = (2..7)
    .flat_map(|x| (2..7).flat_map(move |y| (2..7).map(move |z| Pos(x, y, z))))
    .collect();
}

trait _Solve2: Cube + Sized {
  fn _apply_thin_move(&self, Move(center, axis, amount): Move) {
    let mut offcenter = center;
    offcenter[axis] -= 1;
    let offaxis = if axis == Axis::X { Axis::Y } else { Axis::X };
    self.apply_move(Move(offcenter, axis, amount));
    self.apply_move(Move(center, offaxis, 2));
    self.apply_move(Move(offcenter, axis, -amount));
    self.apply_move(Move(center, offaxis, 2));
    self.apply_move(Move(center, axis, -amount));
  }
  fn _apply_thin_moves(&self, moves: Vec<Move>) {
    for m in moves {
      self._apply_thin_move(m);
    }
  }
  fn _solve_cross(&self) {}
}

impl<T: Cube> _Solve2 for T {}
impl<T: Cube> Solve2 for T {}
