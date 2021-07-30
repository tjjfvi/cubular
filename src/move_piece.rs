use crate::*;

pub trait MovePiece: Cube + Sized {
  fn move_piece(&self, mut from: Pos, to: Pos) {
    if from.parity() != to.parity() {
      dbg!((from, to));
      panic!("Cannot move between positions of different parities");
    }
    while from != to {
      _move_piece_axis(&*self, Axis::X, &mut from, to);
      _move_piece_axis(&*self, Axis::Y, &mut from, to);
      _move_piece_axis(&*self, Axis::Z, &mut from, to);
    }
  }
}

fn _move_piece_axis<C: Cube>(cube: C, axis: Axis, from: &mut Pos, to: Pos) {
  let mapped = cube.swap_axes(Axis::X, axis);
  let mut from2 = from.swap_axes(Axis::X, axis);
  _move_piece_x(mapped, &mut from2, to.swap_axes(Axis::X, axis));
  *from = from2.swap_axes(Axis::X, axis)
}

fn _move_piece_x<C: Cube>(cube: C, from: &mut Pos, to: Pos) {
  let size = cube.size();
  while from.0 != to.0 {
    let mut center = Pos(0, 0, 0);
    let amount;
    if from.0 < to.0 {
      center.0 = from.0 + 1
    } else {
      center.0 = from.0 - 1
    }
    if (from.0 + 1 == to.0 || to.0 + 1 == from.0) && from.0 != 0 && from.0 != size.0 - 1 {
      center.0 = from.0
    }
    if from.1 <= 1 {
      center.1 = from.1 + 1;
      amount = if from.0 < to.0 { 1 } else { 3 }
    } else {
      center.1 = from.1 - 1;
      amount = if from.0 < to.0 { 3 } else { 1 }
    }
    if from.2 <= 1 {
      center.2 = 1
    } else {
      center.2 = from.2 - 1
    }
    cube.apply_move(Move(center, Axis::Z, amount));
    *from = (*from - (center - Pos(1, 1, 1))).rotate(Axis::Z, amount, 3) + (center - Pos(1, 1, 1));
  }
}

impl<T: Cube> MovePiece for T {}
