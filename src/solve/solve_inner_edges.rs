use super::*;
use std::collections::HashSet;

#[derive(Default)]
pub struct SolveInnerEdges(HashSet<Pos>);

impl SolveStep for SolveInnerEdges {
  fn get_solved(&mut self) -> &mut HashSet<Pos> {
    &mut self.0
  }
  fn in_bounds(&mut self, pos: Pos) -> bool {
    true
      && pos.0 >= 3
      && pos.0 <= 5
      && pos.1 >= 3
      && pos.1 <= 5
      && pos.2 >= 3
      && pos.2 <= 5
      && pos.parity() == 0
      && pos != Pos(4, 4, 4)
  }
  fn move_pool<C: Cube>(&mut self, _cube: &C, from: Pos, to: Pos) {
    assert_eq!(from, to)
  }
  fn get_swap<C: Cube>(&mut self, _cube: &C, pos: Pos) -> Option<Swap> {
    let initial_moves = match pos {
      Pos(5, 4, 3) => return None, // once all of the others are solved, this must be too
      Pos(3, 4, 3) => vec![],
      Pos(x, y, 5) => vec![
        Move(
          Pos(4, 4, 5),
          Axis::Z,
          if x == 5 {
            2
          } else if y == 5 {
            1
          } else if y == 3 {
            -1
          } else {
            0
          },
        ),
        Move(Pos(3, 4, 4), Axis::X, 2),
      ],
      Pos(x, y, 4) => vec![
        Move(
          Pos(4, 4, 4),
          Axis::Z,
          if x == 5 {
            if y == 5 {
              1
            } else {
              2
            }
          } else {
            if y == 5 {
              0
            } else {
              -1
            }
          },
        ),
        Move(Pos(3, 4, 4), Axis::X, -1),
      ],
      Pos(4, y, 3) => vec![
        Move(Pos(4, 4, 4), Axis::X, 2),
        Move(Pos(4, 4, 5), Axis::Z, if y == 5 { -1 } else { 1 }),
        Move(Pos(3, 4, 4), Axis::X, 2),
      ],
      _ => panic!("Unreachable"),
    };
    let mut moves = initial_moves.clone();
    moves.extend(vec![
      Move(Pos(5, 4, 4), Axis::X, -1), // R
      Move(Pos(4, 4, 3), Axis::Z, 1),  // U
      Move(Pos(5, 4, 4), Axis::X, 1),  // R'
      Move(Pos(4, 4, 3), Axis::Z, -1), // U'
      Move(Pos(5, 4, 4), Axis::X, 1),  // R'
      Move(Pos(4, 5, 4), Axis::Y, 1),  // F
      Move(Pos(5, 4, 4), Axis::X, 2),  // R2
      Move(Pos(4, 4, 3), Axis::Z, -1), // U'
      Move(Pos(5, 4, 4), Axis::X, 1),  // R'
      Move(Pos(4, 4, 3), Axis::Z, -1), // U'
      Move(Pos(5, 4, 4), Axis::X, -1), // R
      Move(Pos(4, 4, 3), Axis::Z, 1),  // U
      Move(Pos(5, 4, 4), Axis::X, 1),  // R'
      Move(Pos(4, 5, 4), Axis::Y, -1), // F'
    ]);
    moves.extend(initial_moves.reverse_moves());
    Some(Swap {
      index: if pos == Pos(5, 4, 3) { 1 } else { 0 },
      source: Pos(5, 4, 3),
      moves,
    })
  }
  fn apply_move<C: Cube>(&mut self, cube: &C, m: Move) {
    cube.apply_thin_move(m);
  }
}
