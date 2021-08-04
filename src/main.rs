mod apply_move;
mod cube;
mod cube_iter;
mod display_cube;
mod flip;
mod is_solved;
mod n;
mod pos;
mod root_cube;
mod scramble;
mod slice;
mod solve;
mod swap_axes;

use std::time::Instant;

pub(crate) use apply_move::*;
pub(crate) use cube::*;
pub(crate) use cube_iter::*;
pub(crate) use display_cube::*;
pub(crate) use flip::*;
pub(crate) use is_solved::*;
pub(crate) use n::*;
pub(crate) use pos::*;
pub(crate) use root_cube::*;
pub(crate) use scramble::*;
pub(crate) use slice::*;
pub(crate) use solve::*;
pub(crate) use swap_axes::*;

fn main() {
  let start = Instant::now();
  let mut cube = RootCube::solved();
  cube.scramble(1000);
  cube.reset_moves();
  cube.solve();
  cube.print();
  println!("{}", cube.moves.len());
  println!("{:?}", start.elapsed());
}
