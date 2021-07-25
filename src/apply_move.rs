use crate::*;

pub trait ApplyMove: Cube + Sized {
  fn apply_move(&mut self, center: Pos, axis: Axis, amount: i8) {
    let mut view = self.slice(center - Pos(1, 1, 1), Pos(3, 3, 3));
    let mapped: Vec<_> = (&mut view)
      .rotate(axis, amount)
      .iter()
      .map(|x| x.clone())
      .collect();
    for (pos, val) in mapped {
      view.set(pos, val)
    }
  }
}

impl<T: Cube> ApplyMove for T {}
