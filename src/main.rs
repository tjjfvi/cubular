mod apply_move;
mod cube;
mod cube_iter;
mod display_cube;
mod flip;
mod n;
mod pos;
mod root_cube;
mod rotate;
mod scramble;
mod shift;
mod slice;
mod swap_axes;

pub(crate) use apply_move::*;
pub(crate) use cube::*;
pub(crate) use cube_iter::*;
pub(crate) use display_cube::*;
pub(crate) use flip::*;
pub(crate) use n::*;
pub(crate) use pos::*;
pub(crate) use root_cube::*;
pub(crate) use rotate::*;
pub(crate) use scramble::*;
pub(crate) use shift::*;
pub(crate) use slice::*;
pub(crate) use swap_axes::*;

fn main() {
  let mut cube = RootCube::solved();
  cube.scramble(1000);
  println!("{}", DisplayCube::from(cube));
}
