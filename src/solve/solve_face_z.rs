use std::collections::HashSet;

use super::*;

#[derive(Default)]
pub struct SolveFaceZ(HashSet<Pos>);

impl SolveStep for SolveFaceZ {
  fn get_solved(&mut self) -> &mut HashSet<Pos> {
    &mut self.0
  }
  fn get_swap<C: Cube>(&mut self, cube: &C, pos: Pos) -> Option<Swap> {
    let size = cube.size();
    let Pos(x, y, z) = pos;
    if z != 0 {
      return None;
    };
    let index = y * size.0 + x;
    Some(if y + 2 >= size.1 {
      if x + 2 >= size.0 {
        Swap {
          index,
          source: Pos(size.0 - 4 + size.1 - y, size.1 - 1, 5 + x - size.0),
          moves: vec![
            Move(Pos(size.0 - 2, size.1 - 2, 1), Axis::X, 1),
            Move(Pos(size.0 - 2, size.1 - 2, 1), Axis::Y, 1),
            Move(
              Pos(size.0 + size.1 - 5 - y, size.1 - 2, 4 + x - size.0),
              Axis::Y,
              -1,
            ),
            Move(Pos(size.0 - 2, size.1 - 2, 1), Axis::Y, -1),
            Move(Pos(size.0 - 2, size.1 - 2, 1), Axis::X, -1),
          ],
        }
      } else {
        Swap {
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
        Swap {
          index,
          source: Pos(size.0 - 1, y, 5 + x - size.0),
          moves: vec![
            Move(Pos(size.0 - 2, y + 1, 1), Axis::Y, 1),
            Move(Pos(size.0 - 2, y + 1, 4 + x - size.0), Axis::Y, -1),
            Move(Pos(size.0 - 2, y + 1, 1), Axis::Y, -1),
          ],
        }
      } else {
        Swap {
          index,
          source: pos + Pos(0, 0, 2),
          moves: vec![Move(pos + Pos(1, 1, 1), Axis::Y, 1)],
        }
      }
    })
  }
  fn move_pool<C: Cube>(&mut self, cube: &C, from: Pos, to: Pos) {
    cube
      .slice(Pos(0, 0, 1), cube.size() - Pos(0, 0, 1))
      .move_piece(from - Pos(0, 0, 1), to - Pos(0, 0, 1))
  }
}

// fn _solve(&self) {
//   for i in 0..=1 {
//     self._solve_face(Axis::X, Pos(i, i, i));
//     self._solve_face(Axis::Y, Pos(i + 1, i, i));
//     self._solve_face(Axis::Z, Pos(i + 1, i + 1, i));
//   }
//   for i in 0..=1 {
//     self._solve_face_flipped(Axis::X, Pos(i, i, i));
//     self._solve_face_flipped(Axis::Y, Pos(i + 1, i, i));
//     self._solve_face_flipped(Axis::Z, Pos(i + 1, i + 1, i));
//   }
// }
// fn _solve_face(&self, axis: Axis, offset: Pos) {
//   self
//     .slice(offset, self.size() - offset)
//     .swap_axes(Axis::Z, axis)
//     ._solve_face_z()
// }
// fn _solve_face_flipped(&self, axis: Axis, offset: Pos) {
//   self
//     .slice(Pos(2, 2, 2), self.size() - Pos(2, 2, 2) - offset)
//     .swap_axes(Axis::Z, axis)
//     .flip(Axis::Z)
//     ._solve_face_z()
// }
// let (x2, y2) = (y..size.1)
//   .flat_map(|y2| ((if y2 == y { x } else { 0 })..size.0).map(move |x2| (x2, y2)))
//   .find(|(x2, y2)| self.get(Pos(*x2, *y2, 0)) == value)
//   .unwrap();
