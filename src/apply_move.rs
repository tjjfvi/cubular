use crate::*;

pub trait ApplyMove: Cube + Sized {
  fn apply_move(&mut self, center: Pos, axis: Axis, amount: i8) {
    // println!("apply_move{:?}", (center, axis, amount));
    let mut view = self.slice(center - Pos(1, 1, 1), Pos(3, 3, 3));
    let mapped: Vec<_> = (&mut view).rotate(axis, amount).iter().collect();
    for (pos, val) in mapped {
      unsafe { view.set(pos, val) }
    }
  }
}

impl<T: Cube> ApplyMove for T {}
