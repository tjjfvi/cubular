use crate::*;

pub trait ApplyMove: Cube + Sized {
  fn apply_move(&self, center: Pos, axis: Axis, amount: i8) {
    println!("apply_move{:?}", (center, axis, amount));
    let slice = (&*self).slice(center - Pos(1, 1, 1), Pos(3, 3, 3));
    let mapped: Vec<_> = (&slice).rotate(axis, amount).iter().collect();
    for (pos, val) in mapped {
      unsafe { slice.set(pos, val) }
    }
  }
}

impl<T: Cube> ApplyMove for T {}
