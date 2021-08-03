mod apply_move;
mod cube;
mod cube_iter;
mod display_cube;
mod flip;
mod move_piece;
mod move_value;
mod n;
mod pos;
mod root_cube;
mod rotate;
mod scramble;
mod shift;
mod slice;
mod solve_final;
mod solve_initial;
mod swap;
mod swap_axes;

pub(crate) use apply_move::*;
pub(crate) use cube::*;
pub(crate) use cube_iter::*;
pub(crate) use display_cube::*;
pub(crate) use flip::*;
pub(crate) use move_piece::*;
pub(crate) use move_value::*;
pub(crate) use n::*;
pub(crate) use pos::*;
pub(crate) use root_cube::*;
pub(crate) use rotate::*;
pub(crate) use scramble::*;
pub(crate) use shift::*;
pub(crate) use slice::*;
pub(crate) use solve_final::*;
pub(crate) use solve_initial::*;
pub(crate) use swap::*;
pub(crate) use swap_axes::*;

fn main() {
  let cube = RootCube::solved();
  cube.apply_move(Move(Pos(1, 1, 1), Axis::Z, -1));
  cube.solve_initial();
  cube.solve_final();
}

fn is_solved<C: Cube>(cube: C) -> bool {
  cube.iter().all(|(pos, val)| val == cube.get_solved(pos))
}
