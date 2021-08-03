use crate::*;

pub trait IsSolved: Cube {
  fn is_solved(&self) -> bool {
    self.iter().all(|(pos, val)| val == self.get_solved(pos))
  }
}

impl<T: Cube> IsSolved for T {}
