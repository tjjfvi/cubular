include!("lib.rs");

use std::io::{stdin, Read};

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
    // Change to get_compat_move_str to output in the challenge's coordinate system
    println!("{}", get_move_str(&m));
    self.0.apply_move(m);
  }
}
