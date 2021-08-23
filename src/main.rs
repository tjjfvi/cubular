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

use apply_thin_move::*;
use cube::*;
use cube_iter::*;
use display_cube::*;
use flip::*;
use is_solved::*;
use move_str::*;
use parse_cube::*;
use pos::*;
use r#move::*;
use scramble::*;
use slice::*;
use solve::*;
use solve_front_face::*;
use solve_inner_corners::*;
use solve_inner_cross::*;
use solve_inner_edges::*;
use solve_inner_shell::*;
use solve_step::*;
use swap_axes::*;
use value::*;

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
