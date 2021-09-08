use crate::*;

pub struct SolveInnerCross;
impl SolveStep for SolveInnerCross {
  fn in_bounds(&self, pos: Pos) -> bool {
    pos.0 >= 1
      && pos.0 <= 3
      && pos.1 >= 1
      && pos.1 <= 3
      && pos.2 >= 1
      && pos.2 <= 3
      && pos.parity() == 1
  }
  fn move_pool<C: Cube>(&self, cube: &mut C, from: Pos, to: Pos) {
    fn f(pos: Pos) -> i8 {
      if pos.0 == 1 {
        if pos.1 == 1 {
          0
        } else {
          1
        }
      } else {
        if pos.1 == 1 {
          3
        } else {
          2
        }
      }
    }
    cube.apply_thin_move(Move(Pos(2, 2, from.2), Axis::Z, f(from) - f(to)));
    if to.2 != from.2 {
      cube.apply_thin_move(Move(Pos(2, to.1, 2), Axis::Y, (to.0 + to.2 - 1) as i8 % 4));
    }
  }
  fn get_swap<C: Cube>(&self, _cube: &C, pos: Pos) -> Option<Swap> {
    match pos {
      Pos(x, 2, 2) => Some(Swap {
        index: 0,
        source: Pos(x, 1, 1),
        moves: vec![
          Move(Pos(x, 2, 1), Axis::X, 1),
          Move(Pos(x, 1, 2), Axis::X, -1),
          Move(Pos(x, 2, 1), Axis::X, -1),
          Move(Pos(x, 1, 2), Axis::X, 1),
        ],
      }),
      Pos(2, y, 2) => Some(Swap {
        index: 0,
        source: Pos(1, y, 1),
        moves: vec![
          Move(Pos(2, y, 1), Axis::Y, 1),
          Move(Pos(1, y, 2), Axis::Y, -1),
          Move(Pos(2, y, 1), Axis::Y, -1),
          Move(Pos(1, y, 2), Axis::Y, 1),
        ],
      }),
      Pos(2, 2, z) => Some(Swap {
        index: 0,
        source: Pos(1, 1, z),
        moves: vec![
          Move(Pos(2, 1, z), Axis::Z, 1),
          Move(Pos(1, 2, z), Axis::Z, -1),
          Move(Pos(2, 1, z), Axis::Z, -1),
          Move(Pos(1, 2, z), Axis::Z, 1),
        ],
      }),
      _ => None,
    }
  }
  fn apply_move<C: Cube>(&self, cube: &mut C, m: Move) {
    cube.apply_thin_move(m);
  }
}
