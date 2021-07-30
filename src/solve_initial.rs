use crate::*;

pub trait SolveInitial: Cube + Sized {
  fn solve_initial(&self) {
    self._solve()
  }
}

trait _SolveInitial: Cube + Sized {
  fn _solve(&self) {
    for i in 0..4 {
      self._solve_face(Axis::X, Pos(i, i, i));
      self._solve_face(Axis::Y, Pos(i + 1, i, i));
      self._solve_face(Axis::Z, Pos(i + 1, i + 1, i));
    }
  }
  fn _solve_face(&self, axis: Axis, offset: Pos) {
    self
      .slice(offset, self.size() - offset)
      .swap_axes(Axis::Z, axis)
      ._solve_face_z()
  }
  fn _solve_face_z(&self) {
    let size = self.size();
    let pool = self.slice(Pos(0, 0, 1), size - Pos(0, 0, 1));
    for y in 0..size.1 {
      for x in 0..size.0 {
        let pos = Pos(x, y, 0);
        let value = self.get_solved(pos);
        if self.get(pos) == value {
          continue;
        }
        let swap = self._get_swap(x, y);
        if pool.move_value(value, swap.source - Pos(0, 0, 1)).is_none() {
          let (x2, y2) = (y..size.1)
            .flat_map(|y2| ((if y2 == y { x } else { 0 })..size.0).map(move |x2| (x2, y2)))
            .find(|(x2, y2)| self.get(Pos(*x2, *y2, 0)) == value)
            .unwrap();
          let swap2 = self._get_swap(x2, y2);
          self.apply_moves(swap2.moves.reverse_moves());
          pool.move_piece(swap2.source - Pos(0, 0, 1), swap.source - Pos(0, 0, 1));
        }
        self.apply_moves(swap.moves);
      }
    }
  }
  fn _get_swap(&self, x: usize, y: usize) -> Swap {
    let size = self.size();
    let pos = Pos(x, y, 0);
    if y + 2 >= size.1 {
      if x + 2 >= size.0 {
        Swap {
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
          source: Pos(size.0 - 1, y, 5 + x - size.0),
          moves: vec![
            Move(Pos(size.0 - 2, y + 1, 1), Axis::Y, 1),
            Move(Pos(size.0 - 2, y + 1, 4 + x - size.0), Axis::Y, -1),
            Move(Pos(size.0 - 2, y + 1, 1), Axis::Y, -1),
          ],
        }
      } else {
        Swap {
          source: pos + Pos(0, 0, 2),
          moves: vec![Move(pos + Pos(1, 1, 1), Axis::Y, 1)],
        }
      }
    }
  }
}

#[derive(Debug)]
struct Swap {
  source: Pos,
  moves: Vec<Move>,
}

impl<T: Cube> _SolveInitial for T {}
impl<T: Cube> SolveInitial for T {}
