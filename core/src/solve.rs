use crate::*;

pub trait Solve: Cube + Sized {
  fn solve(&mut self) {
    let size = self.size();
    let mut min = Pos(0, 0, 0);
    let mut max = size;
    for &axis in [Axis::Z, Axis::Y, Axis::X].iter() {
      for _ in 0..((size[axis] - 5 + 1/* round up */) / 2) {
        self._solve_face(axis, min, max);
        min[axis] += 1;
      }
      for _ in 0..((size[axis] - 5) / 2) {
        self._solve_face_flipped(axis, min, max);
        max[axis] -= 1;
      }
    }
    self
      .slice(min, Pos(5, 5, 5))
      .apply_solve_step(SolveInnerShell)
      .apply_solve_step(SolveInnerCross)
      .apply_solve_step(SolveInnerEdges)
      .apply_solve_step(SolveInnerCorners);
  }
}

trait _Solve: Cube {
  fn _solve_face(&mut self, axis: Axis, min: Pos, max: Pos) {
    self
      .slice(min, max - min)
      .swap_axes(Axis::Z, axis)
      .apply_solve_step(SolveFrontFace);
  }
  fn _solve_face_flipped(&mut self, axis: Axis, min: Pos, max: Pos) {
    self
      .slice(min, max - min)
      .swap_axes(Axis::Z, axis)
      .flip(Axis::X)
      .flip(Axis::Y)
      .flip(Axis::Z)
      .apply_solve_step(SolveFrontFace);
  }
}

impl<T: Cube> Solve for T {}
impl<T: Cube> _Solve for T {}
