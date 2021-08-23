use crate::*;

pub struct SolveInnerEdges;
impl SolveStep for SolveInnerEdges {
  fn move_pool<C: Cube>(&self, _cube: &mut C, from: Pos, to: Pos) {
    assert_eq!(from, to)
  }
  fn classify<C: Cube>(&self, _cube: &C, pos: Pos) -> PosClass {
    let in_bounds =
      pos.within(Pos(1, 1, 1), Pos(3, 3, 3)) && pos.parity() == 0 && pos != Pos(2, 2, 2);
    if !in_bounds {
      return PosClass::Other;
    }
    let initial_moves = match pos {
      Pos(3, 2, 1) => return PosClass::Pool, // once all of the others are solved, this must be too
      Pos(1, 2, 1) => vec![],
      Pos(x, y, 3) => vec![
        Move(
          Pos(2, 2, 3),
          Axis::Z,
          if x == 3 {
            2
          } else if y == 3 {
            1
          } else if y == 1 {
            -1
          } else {
            0
          },
        ),
        Move(Pos(1, 2, 2), Axis::X, 2),
      ],
      Pos(x, y, 2) => vec![
        Move(
          Pos(2, 2, 2),
          Axis::Z,
          if x == 3 {
            if y == 3 {
              1
            } else {
              2
            }
          } else {
            if y == 3 {
              0
            } else {
              -1
            }
          },
        ),
        Move(Pos(1, 2, 2), Axis::X, -1),
      ],
      Pos(2, y, 1) => vec![
        Move(Pos(2, 2, 2), Axis::X, 2),
        Move(Pos(2, 2, 3), Axis::Z, if y == 3 { -1 } else { 1 }),
        Move(Pos(1, 2, 2), Axis::X, 2),
      ],
      _ => panic!("Unreachable"),
    };
    let mut moves = initial_moves.clone();
    moves.extend_from_slice(&T_PERMUTATION);
    moves.extend(initial_moves.reverse_moves());
    PosClass::Active {
      index: if pos == Pos(3, 2, 1) { 1 } else { 0 },
      source: Pos(3, 2, 1),
      moves,
    }
  }
  fn apply_move<C: Cube>(&self, cube: &mut C, m: Move) {
    cube.apply_thin_move(m);
  }
}

static T_PERMUTATION: [Move; 14] = [
  Move(Pos(3, 2, 2), Axis::X, -1), // R
  Move(Pos(2, 2, 1), Axis::Z, 1),  // U
  Move(Pos(3, 2, 2), Axis::X, 1),  // R'
  Move(Pos(2, 2, 1), Axis::Z, -1), // U'
  Move(Pos(3, 2, 2), Axis::X, 1),  // R'
  Move(Pos(2, 3, 2), Axis::Y, 1),  // F
  Move(Pos(3, 2, 2), Axis::X, 2),  // R2
  Move(Pos(2, 2, 1), Axis::Z, -1), // U'
  Move(Pos(3, 2, 2), Axis::X, 1),  // R'
  Move(Pos(2, 2, 1), Axis::Z, -1), // U'
  Move(Pos(3, 2, 2), Axis::X, -1), // R
  Move(Pos(2, 2, 1), Axis::Z, 1),  // U
  Move(Pos(3, 2, 2), Axis::X, 1),  // R'
  Move(Pos(2, 3, 2), Axis::Y, -1), // F'
];
