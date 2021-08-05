mod apply_thin_move;
mod solve_front_face;
mod solve_inner_corners;
mod solve_inner_cross;
mod solve_inner_edges;
mod solve_outer_shell;
mod solve_step;
mod swap;

pub(self) use crate::*;
pub(self) use apply_thin_move::*;
pub(self) use solve_front_face::*;
pub(self) use solve_inner_corners::*;
pub(self) use solve_inner_cross::*;
pub(self) use solve_inner_edges::*;
pub(self) use solve_outer_shell::*;
pub(self) use solve_step::*;
pub(self) use swap::*;

pub trait Solve: Cube + Sized {
  fn solve(&mut self) {
    self._solve_face(Axis::Z, Pos(0, 0, 0), Pos(9, 9, 9));
    self._solve_face(Axis::Z, Pos(0, 0, 1), Pos(9, 9, 9));
    self._solve_face_flipped(Axis::Z, Pos(0, 0, 2), Pos(9, 9, 9));
    self._solve_face_flipped(Axis::Z, Pos(0, 0, 2), Pos(9, 9, 8));
    self._solve_face(Axis::Y, Pos(0, 0, 2), Pos(9, 9, 7));
    self._solve_face(Axis::Y, Pos(0, 1, 2), Pos(9, 9, 7));
    self._solve_face_flipped(Axis::Y, Pos(0, 2, 2), Pos(9, 9, 7));
    self._solve_face_flipped(Axis::Y, Pos(0, 2, 2), Pos(9, 8, 7));
    self._solve_face(Axis::X, Pos(0, 2, 2), Pos(9, 7, 7));
    self._solve_face(Axis::X, Pos(1, 2, 2), Pos(9, 7, 7));
    self._solve_face_flipped(Axis::X, Pos(2, 2, 2), Pos(9, 7, 7));
    self._solve_face_flipped(Axis::X, Pos(2, 2, 2), Pos(8, 7, 7));
    self
      .slice(Pos(2, 2, 2), Pos(5, 5, 5))
      .apply_solve_step(SolveOuterShell);
    self.apply_solve_step(SolveInnerCross);
    self.apply_solve_step(SolveInnerEdges);
    self.apply_solve_step(SolveInnerCorners);
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
      .flip(Axis::Z)
      .apply_solve_step(SolveFrontFace);
  }
}

impl<T: Cube> Solve for T {}
impl<T: Cube> _Solve for T {}
