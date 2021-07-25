use rand::Rng;

use crate::*;

pub trait Scramble: Cube + Sized {
  fn scramble(&mut self, iterations: u32) {
    let mut rng = rand::thread_rng();
    let size = self.size();
    for _ in 0..iterations {
      let center = Pos(
        rng.gen_range(1..size.0 - 1),
        rng.gen_range(1..size.1 - 1),
        rng.gen_range(1..size.2 - 1),
      );
      let axis = match rng.gen_range(0..3) {
        0 => Axis::X,
        1 => Axis::Y,
        _ => Axis::Z,
      };
      let amount = rng.gen_range(1..3);
      self.apply_move(center, axis, amount);
    }
  }
}

impl<T: Cube> Scramble for T {}
