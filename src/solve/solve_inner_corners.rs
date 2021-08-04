use super::*;

pub struct SolveInnerCorners;
impl SolveStep for SolveInnerCorners {
  fn in_bounds(&self, pos: Pos) -> bool {
    true
      && pos.0 >= 3
      && pos.0 <= 5
      && pos.1 >= 3
      && pos.1 <= 5
      && pos.2 >= 3
      && pos.2 <= 5
      && pos.parity() == 1
      && pos.0 != 4
      && pos.1 != 4
      && pos.2 != 4
  }
  fn move_pool<C: Cube>(&self, _cube: &mut C, from: Pos, to: Pos) {
    assert_eq!(from, to)
  }
  fn get_swap<C: Cube>(&self, _cube: &C, pos: Pos) -> Option<Swap> {
    let initial_moves = match pos {
      Pos(3, 3, 3) => return None, // once all of the others are solved, this must be too
      Pos(5, 5, 3) => vec![],
      Pos(3, 5, 3) => vec![Move(Pos(4, 5, 4), Axis::Y, 1)],
      Pos(5, 3, 3) => vec![Move(Pos(5, 4, 4), Axis::X, 1)],
      Pos(x, y, 5) => vec![
        Move(
          Pos(4, 4, 5),
          Axis::Z,
          if x == 3 {
            if y == 3 {
              2
            } else {
              -1
            }
          } else {
            if y == 3 {
              1
            } else {
              0
            }
          },
        ),
        Move(Pos(4, 5, 4), Axis::Y, -1),
      ],
      _ => panic!("Unreachable"),
    };
    let mut moves = initial_moves.clone();
    moves.extend_from_slice(&Y_PERMUTATION);
    moves.extend(initial_moves.reverse_moves());
    Some(Swap {
      index: if pos == Pos(5, 4, 3) { 1 } else { 0 },
      source: Pos(3, 3, 3),
      moves,
    })
  }
  fn apply_move<C: Cube>(&self, cube: &mut C, m: Move) {
    cube.apply_thin_move(m);
  }
}

static Y_PERMUTATION: [Move; 17] = [
  Move(Pos(4, 5, 4), Axis::Y, 1),  // F
  Move(Pos(5, 4, 4), Axis::X, -1), // R
  Move(Pos(4, 4, 3), Axis::Z, -1), // U'
  Move(Pos(5, 4, 4), Axis::X, 1),  // R'
  Move(Pos(4, 4, 3), Axis::Z, -1), // U'
  Move(Pos(5, 4, 4), Axis::X, -1), // R
  Move(Pos(4, 4, 3), Axis::Z, 1),  // U
  Move(Pos(5, 4, 4), Axis::X, 1),  // R'
  Move(Pos(4, 5, 4), Axis::Y, -1), // F'
  Move(Pos(5, 4, 4), Axis::X, -1), // R
  Move(Pos(4, 4, 3), Axis::Z, 1),  // U
  Move(Pos(5, 4, 4), Axis::X, 1),  // R'
  Move(Pos(4, 4, 3), Axis::Z, -1), // U'
  Move(Pos(5, 4, 4), Axis::X, 1),  // R'
  Move(Pos(4, 5, 4), Axis::Y, 1),  // F
  Move(Pos(5, 4, 4), Axis::X, -1), // R
  Move(Pos(4, 5, 4), Axis::Y, -1), // F'
];
