use crate::*;

pub struct SolveInnerCross;
impl SolveStep for SolveInnerCross {
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
      cube.apply_thin_move(Move(Pos(2, to.1, 2), Axis::Y, -((to.0 + to.2 - 1) as i8)));
    }
  }
  fn classify<C: Cube>(&self, _cube: &C, pos: Pos) -> PosClass {
    let in_bounds = pos.within(Pos(1, 1, 1), Pos(3, 3, 3)) && pos.parity() == 1;
    if !in_bounds {
      return PosClass::Other;
    }
    match pos {
      Pos(x, 2, 2) => PosClass::Active {
        index: 0,
        source: Pos(x, 1, 1),
        moves: vec![
          Move(Pos(x, 2, 1), Axis::X, 1),
          Move(Pos(x, 1, 2), Axis::X, -1),
          Move(Pos(x, 2, 1), Axis::X, -1),
          Move(Pos(x, 1, 2), Axis::X, 1),
        ],
      },
      Pos(2, y, 2) => PosClass::Active {
        index: 0,
        source: Pos(1, y, 1),
        moves: vec![
          Move(Pos(1, y, 2), Axis::Y, 1),
          Move(Pos(2, y, 1), Axis::Y, -1),
          Move(Pos(1, y, 2), Axis::Y, -1),
          Move(Pos(2, y, 1), Axis::Y, 1),
        ],
      },
      Pos(2, 2, z) => PosClass::Active {
        index: 0,
        source: Pos(1, 1, z),
        moves: vec![
          Move(Pos(2, 1, z), Axis::Z, 1),
          Move(Pos(1, 2, z), Axis::Z, -1),
          Move(Pos(2, 1, z), Axis::Z, -1),
          Move(Pos(1, 2, z), Axis::Z, 1),
        ],
      },
      _ => PosClass::Pool,
    }
  }
  fn apply_move<C: Cube>(&self, cube: &mut C, m: Move) {
    cube.apply_thin_move(m);
  }
}
