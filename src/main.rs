mod cube;
mod cube_iter;
mod display_cube;
mod flip;
mod is_solved;
mod r#move;
mod parse_cube;
mod pos;
mod root_cube;
mod scramble;
mod slice;
mod solve;
mod swap_axes;
mod value;

use std::time::Instant;

pub(crate) use cube::*;
pub(crate) use cube_iter::*;
pub(crate) use display_cube::*;
pub(crate) use flip::*;
pub(crate) use is_solved::*;
pub(crate) use parse_cube::*;
pub(crate) use pos::*;
pub(crate) use r#move::*;
pub(crate) use root_cube::*;
pub(crate) use scramble::*;
pub(crate) use slice::*;
pub(crate) use solve::*;
pub(crate) use swap_axes::*;
pub(crate) use value::*;

fn main() {
  let start = Instant::now();
  let mut cube = RootCube::solved();
  for _ in 0..1000 {
    cube.scramble(1000);
    cube.reset_moves();
    cube.solve();
    cube.reset_moves();
    assert!(cube.is_solved());
  }
  println!("{:?}", start.elapsed());
}
