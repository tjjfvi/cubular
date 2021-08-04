use super::*;

pub struct SolveInnerCross;
impl SolveStep for SolveInnerCross {
  fn in_bounds(&self, pos: Pos) -> bool {
    pos.0 >= 3
      && pos.0 <= 5
      && pos.1 >= 3
      && pos.1 <= 5
      && pos.2 >= 3
      && pos.2 <= 5
      && pos.parity() == 1
  }
  fn move_pool<C: Cube>(&self, cube: &mut C, from: Pos, to: Pos) {
    fn f(pos: Pos) -> i8 {
      if pos.0 == 3 {
        if pos.1 == 3 {
          0
        } else {
          1
        }
      } else {
        if pos.1 == 3 {
          3
        } else {
          2
        }
      }
    }
    cube.apply_thin_move(Move(Pos(4, 4, from.2), Axis::Z, f(from) - f(to)));
    if to.2 != from.2 {
      cube.apply_thin_move(Move(Pos(4, to.1, 4), Axis::Y, (to.0 + to.2 - 1) as i8 % 4));
    }
  }
  fn get_swap<C: Cube>(&self, _cube: &C, pos: Pos) -> Option<Swap> {
    match pos {
      Pos(x, 4, 4) => Some(Swap {
        index: 0,
        source: Pos(x, 3, 3),
        moves: vec![
          Move(Pos(x, 4, 3), Axis::X, 1),
          Move(Pos(x, 3, 4), Axis::X, -1),
          Move(Pos(x, 4, 3), Axis::X, -1),
          Move(Pos(x, 3, 4), Axis::X, 1),
        ],
      }),
      Pos(4, y, 4) => Some(Swap {
        index: 0,
        source: Pos(3, y, 3),
        moves: vec![
          Move(Pos(4, y, 3), Axis::Y, 1),
          Move(Pos(3, y, 4), Axis::Y, -1),
          Move(Pos(4, y, 3), Axis::Y, -1),
          Move(Pos(3, y, 4), Axis::Y, 1),
        ],
      }),
      Pos(4, 4, z) => Some(Swap {
        index: 0,
        source: Pos(3, 3, z),
        moves: vec![
          Move(Pos(4, 3, z), Axis::Z, 1),
          Move(Pos(3, 4, z), Axis::Z, -1),
          Move(Pos(4, 3, z), Axis::Z, -1),
          Move(Pos(3, 4, z), Axis::Z, 1),
        ],
      }),
      _ => None,
    }
  }
  fn apply_move<C: Cube>(&self, cube: &mut C, m: Move) {
    cube.apply_thin_move(m);
  }
}
