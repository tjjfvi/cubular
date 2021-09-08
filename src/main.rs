mod apply_thin_move;
mod cube;
mod cube_iter;
mod display_cube;
mod flip;
mod is_solved;
mod r#move;
mod move_str;
mod parse_cube;
mod pos;
mod scramble;
mod slice;
mod solve;
mod solve_front_face;
mod solve_inner_corners;
mod solve_inner_cross;
mod solve_inner_edges;
mod solve_inner_shell;
mod solve_step;
mod swap_axes;
mod value;

use std::io::{stdin, Read};

pub use apply_thin_move::*;
pub use cube::*;
pub use cube_iter::*;
pub use flip::*;
pub use is_solved::*;
pub use move_str::*;
pub use parse_cube::*;
pub use pos::*;
pub use r#move::*;
pub use scramble::*;
pub use slice::*;
pub use solve::*;
pub use solve_front_face::*;
pub use solve_inner_corners::*;
pub use solve_inner_cross::*;
pub use solve_inner_edges::*;
pub use solve_inner_shell::*;
pub use solve_step::*;
pub use swap_axes::*;
pub use value::*;

fn main() {
  let mut input = String::new();
  stdin().read_to_string(&mut input).unwrap();
  let mut cube = LoggingCube(parse_cube(&input[..]).unwrap());
  cube.solve();
  assert!(cube.is_solved());
}

struct LoggingCube([[[Value; 9]; 9]; 9]);

impl Cube for LoggingCube {
  fn get(&self, pos: Pos) -> Value {
    self.0.get(pos)
  }
  fn get_solved(&self, pos: Pos) -> Value {
    self.0.get_solved(pos)
  }
  fn size(&self) -> Pos {
    self.0.size()
  }
  fn apply_move(&mut self, m: Move) {
    println!("{}", get_move_str(&m));
    self.0.apply_move(m);
  }
}
