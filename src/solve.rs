// mod apply_thin_move;
mod solve_front_face;
// mod solve_inner_corners;
// mod solve_inner_cross;
// mod solve_inner_edges;
// mod solve_outer_shell;
mod solve_step;
mod swap;

pub(self) use crate::*;
// pub(self) use apply_thin_move::*;
pub(self) use solve_front_face::*;
// pub(self) use solve_inner_corners::*;
// pub(self) use solve_inner_cross::*;
// pub(self) use solve_inner_edges::*;
// pub(self) use solve_outer_shell::*;
pub(self) use solve_step::*;
pub(self) use swap::*;

pub trait Solve: Cube + Sized {
  fn solve(&mut self) {
    // for i in 0..=1 {
    self.apply_solve_step::<SolveFrontFace>()
    // let i = 0;
    // self._solve_face(Axis::X, Pos(i, i, i));
    //   self._solve_face(Axis::Y, Pos(i + 1, i, i));
    //   self._solve_face(Axis::Z, Pos(i + 1, i + 1, i));
    // }
    // for i in 0..=1 {
    //   self._solve_face_flipped(Axis::X, Pos(i, i, i));
    //   self._solve_face_flipped(Axis::Y, Pos(i + 1, i, i));
    //   self._solve_face_flipped(Axis::Z, Pos(i + 1, i + 1, i));
    // }
    // self
    //   .slice(Pos(2, 2, 2), Pos(5, 5, 5))
    //   .apply_solve_step::<SolveOuterShell>();
    // self.apply_solve_step::<SolveInnerCross>();
    // self.apply_solve_step::<SolveInnerEdges>();
    // self.apply_solve_step::<SolveInnerCorners>();
  }
}

// trait _Solve: Cube {
//   fn _solve_face(&mut self, axis: Axis, offset: Pos) {
//     self
//       .slice(offset, self.size() - offset)
//       .swap_axes(Axis::Z, axis)
//       .apply_solve_step::<SolveFrontFace>()
//   }
//   fn _solve_face_flipped(&mut self, axis: Axis, offset: Pos) {
//     self
//       .slice(Pos(2, 2, 2), self.size() - Pos(2, 2, 2) - offset)
//       .swap_axes(Axis::Z, axis)
//       .flip(Axis::Z)
//       .apply_solve_step::<SolveFrontFace>()
//   }
// }

impl<T: Cube> Solve for T {}
// impl<T: Cube> _Solve for T {}
