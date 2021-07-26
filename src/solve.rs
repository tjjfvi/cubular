use crate::*;

pub trait Solve: Cube + Sized {
  fn solve(&self) {
    self._solve()
  }
}

trait _Solve: Cube + Sized {
  fn _solve(&self) {
    self._solve_face()
  }
  fn _solve_face(&self) {
    let size = self.size();
    let pool = self.slice(Pos(0, 0, 1), size - Pos(0, 0, 1));
    'main: for y in 0..size.1 {
      for x in 0..size.0 {
        let pos = Pos(x, y, 0);
        let value = self.get_solved(pos);
        if self.get(pos) == value {
          continue;
        }
        if y + 2 >= size.1 {
          if x + 2 >= size.0 {
            if pool
              .move_value(
                value,
                Pos(size.0 - 4 + size.1 - y, size.1 - 1, 4 + x - size.0),
              )
              .is_some()
            {
              self.apply_move(Pos(size.0 - 2, size.1 - 2, 1), Axis::X, 1);
              self.apply_move(Pos(size.0 - 2, size.1 - 2, 1), Axis::Y, 1);
              self.apply_move(
                Pos(size.0 - 5 + size.1 - y, size.1 - 2, 4 + x - size.0),
                Axis::Y,
                -1,
              );
              self.apply_move(Pos(size.0 - 2, size.1 - 2, 1), Axis::Y, -1);
              self.apply_move(Pos(size.0 - 2, size.1 - 2, 1), Axis::X, -1);
            }
          } else {
            if pool
              .move_value(value, Pos(x, size.1 - 1, 4 + y - size.1))
              .is_some()
            {
              self.apply_move(Pos(x + 1, size.1 - 2, 1), Axis::X, 1);
              self.apply_move(Pos(x + 1, size.1 - 2, 4 + y - size.1), Axis::X, -1);
              self.apply_move(Pos(x + 1, size.1 - 2, 1), Axis::X, -1);
            }
          }
        } else {
          if x + 2 >= size.0 {
            if pool
              .move_value(value, Pos(size.0 - 1, y, 4 + x - size.0))
              .is_some()
            {
              self.apply_move(Pos(size.0 - 2, y + 1, 1), Axis::Y, 1);
              self.apply_move(Pos(size.0 - 2, y + 1, 4 + x - size.0), Axis::Y, -1);
              self.apply_move(Pos(size.0 - 2, y + 1, 1), Axis::Y, -1);
            }
          } else {
            if pool.move_value(value, pos + Pos(0, 0, 1)).is_some() {
              self.apply_move(pos + Pos(1, 1, 1), Axis::Y, 1);
            }
          }
        }
      }
    }
  }
}

impl<T: Cube> _Solve for T {}
impl<T: Cube> Solve for T {}
