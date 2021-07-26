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
mod solve;
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
pub(crate) use solve::*;
pub(crate) use swap_axes::*;

fn main() {
  let cube = RootCube::solved();
  cube.scramble(1000);
  cube.print();
  cube.solve();
  cube.print();
  // for _ in 0..1000 {
  //   let mut cube = RootCube::solved();
  //   let pos = cube.random_pos();
  //   let val = cube.random_value(pos);
  //   cube.scramble(1000);
  //   // cube.print();
  //   cube.move_value(val, pos).unwrap();
  //   // cube.print();
  //   assert_eq!(cube.get(pos), val);
  // }
}
