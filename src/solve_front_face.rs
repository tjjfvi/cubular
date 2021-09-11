use crate::*;

pub struct SolveFrontFace;
impl SolveStep for SolveFrontFace {
  fn classify<C: Cube>(&self, cube: &C, pos: Pos) -> PosClass {
    let size = cube.size();
    let Pos(x, y, z) = pos;
    if z != 0 {
      return PosClass::Pool;
    };
    let index = y * size.0 + x;
    if y + 2 >= size.1 {
      if x + 2 >= size.0 {
        PosClass::Active {
          index,
          source: Pos(size.0 - 4 + size.1 - y, size.1 - 1, 5 + x - size.0),
          moves: vec![
            Move(Pos(size.0 - 2, size.1 - 2, 1), Axis::X, 1),
            Move(Pos(size.0 - 2, size.1 - 2, 1), Axis::Y, -1),
            Move(
              Pos(size.0 + size.1 - 5 - y, size.1 - 2, 4 + x - size.0),
              Axis::Y,
              1,
            ),
            Move(Pos(size.0 - 2, size.1 - 2, 1), Axis::Y, 1),
            Move(Pos(size.0 - 2, size.1 - 2, 1), Axis::X, -1),
          ],
        }
      } else {
        PosClass::Active {
          index,
          source: Pos(x, size.1 - 1, 5 + y - size.1),
          moves: vec![
            Move(Pos(x + 1, size.1 - 2, 1), Axis::X, 1),
            Move(Pos(x + 1, size.1 - 2, 4 + y - size.1), Axis::X, -1),
            Move(Pos(x + 1, size.1 - 2, 1), Axis::X, -1),
          ],
        }
      }
    } else {
      if x + 2 >= size.0 {
        PosClass::Active {
          index,
          source: Pos(size.0 - 1, y, 5 + x - size.0),
          moves: vec![
            Move(Pos(size.0 - 2, y + 1, 1), Axis::Y, -1),
            Move(Pos(size.0 - 2, y + 1, 4 + x - size.0), Axis::Y, 1),
            Move(Pos(size.0 - 2, y + 1, 1), Axis::Y, 1),
          ],
        }
      } else {
        PosClass::Active {
          index,
          source: pos + Pos(0, 0, 2),
          moves: vec![Move(pos + Pos(1, 1, 1), Axis::Y, -1)],
        }
      }
    }
  }
  fn move_pool<C: Cube>(&self, cube: &mut C, from: Pos, to: Pos) {
    _move_piece(
      cube.slice(Pos(0, 0, 1), cube.size() - Pos(0, 0, 1)),
      from - Pos(0, 0, 1),
      to - Pos(0, 0, 1),
    );
  }
}

fn _move_piece<C: Cube>(mut cube: C, mut from: Pos, to: Pos) {
  if from.parity() != to.parity() {
    panic!("Cannot move between positions of different parities");
  }
  while from != to {
    _move_piece_axis(&mut cube, Axis::X, &mut from, to);
    _move_piece_axis(&mut cube, Axis::Y, &mut from, to);
    _move_piece_axis(&mut cube, Axis::Z, &mut from, to);
  }
}

fn _move_piece_axis<C: Cube>(cube: C, axis: Axis, from: &mut Pos, to: Pos) {
  let mapped = cube.swap_axes(Axis::X, axis);
  let mut from2 = from.swap_axes(Axis::X, axis);
  _move_piece_x(mapped, &mut from2, to.swap_axes(Axis::X, axis));
  *from = from2.swap_axes(Axis::X, axis)
}

fn _move_piece_x<C: Cube>(mut cube: C, from: &mut Pos, to: Pos) {
  let val = cube.get(*from);
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
    debug_assert_eq!(cube.get(*from), val);
  }
}
