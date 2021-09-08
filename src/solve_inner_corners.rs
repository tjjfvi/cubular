use crate::*;

pub struct SolveInnerCorners;
impl SolveStep for SolveInnerCorners {
  fn move_pool<C: Cube>(&self, _cube: &mut C, from: Pos, to: Pos) {
    assert_eq!(from, to)
  }
  fn classify<C: Cube>(&self, _cube: &C, pos: Pos) -> PosClass {
    let in_bounds = true
      && pos.0 >= 1
      && pos.0 <= 3
      && pos.1 >= 1
      && pos.1 <= 3
      && pos.2 >= 1
      && pos.2 <= 3
      && pos.parity() == 1
      && pos.0 != 2
      && pos.1 != 2
      && pos.2 != 2;
    if !in_bounds {
      return PosClass::Other;
    }
    let initial_moves = match pos {
      Pos(1, 1, 1) => return PosClass::Pool, // once all of the others are solved, this must be too
      Pos(3, 3, 1) => vec![],
      Pos(1, 3, 1) => vec![Move(Pos(2, 3, 2), Axis::Y, 1)],
      Pos(3, 1, 1) => vec![Move(Pos(3, 2, 2), Axis::X, 1)],
      Pos(x, y, 3) => vec![
        Move(
          Pos(2, 2, 3),
          Axis::Z,
          if x == 1 {
            if y == 1 {
              2
            } else {
              -1
            }
          } else {
            if y == 1 {
              1
            } else {
              0
            }
          },
        ),
        Move(Pos(2, 3, 2), Axis::Y, -1),
      ],
      _ => unreachable!(),
    };
    let mut moves = initial_moves.clone();
    moves.extend_from_slice(&Y_PERMUTATION);
    moves.extend(initial_moves.reverse_moves());
    PosClass::Active {
      index: if pos == Pos(3, 2, 1) { 1 } else { 0 },
      source: Pos(1, 1, 1),
      moves,
    }
  }
  fn apply_move<C: Cube>(&self, cube: &mut C, m: Move) {
    cube.apply_thin_move(m);
  }
}

static Y_PERMUTATION: [Move; 17] = [
  Move(Pos(2, 3, 2), Axis::Y, 1),  // F
  Move(Pos(3, 2, 2), Axis::X, -1), // R
  Move(Pos(2, 2, 1), Axis::Z, -1), // U'
  Move(Pos(3, 2, 2), Axis::X, 1),  // R'
  Move(Pos(2, 2, 1), Axis::Z, -1), // U'
  Move(Pos(3, 2, 2), Axis::X, -1), // R
  Move(Pos(2, 2, 1), Axis::Z, 1),  // U
  Move(Pos(3, 2, 2), Axis::X, 1),  // R'
  Move(Pos(2, 3, 2), Axis::Y, -1), // F'
  Move(Pos(3, 2, 2), Axis::X, -1), // R
  Move(Pos(2, 2, 1), Axis::Z, 1),  // U
  Move(Pos(3, 2, 2), Axis::X, 1),  // R'
  Move(Pos(2, 2, 1), Axis::Z, -1), // U'
  Move(Pos(3, 2, 2), Axis::X, 1),  // R'
  Move(Pos(2, 3, 2), Axis::Y, 1),  // F
  Move(Pos(3, 2, 2), Axis::X, -1), // R
  Move(Pos(2, 3, 2), Axis::Y, -1), // F'
];
