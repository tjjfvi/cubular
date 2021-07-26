use crate::*;

pub struct Rotate<C: Cube> {
  pub cube: C,
  pub axis: Axis,
  pub amount: i8,
}

impl<C: Cube> Cube for Rotate<C> {
  fn get(&self, pos: Pos) -> N {
    self
      .cube
      .get(pos.rotate(self.axis, -self.amount, self.size().0))
  }
  fn get_solved(&self, pos: Pos) -> N {
    self
      .cube
      .get_solved(pos.rotate(self.axis, -self.amount, self.size().0))
  }
  unsafe fn set(&self, pos: Pos, val: N) {
    self
      .cube
      .set(pos.rotate(self.axis, -self.amount, self.cube.size().0), val)
  }
  fn size(&self) -> Pos {
    self.cube.size()
  }
}

pub trait MakeRotate: Cube + Sized {
  fn rotate(self, axis: Axis, amount: i8) -> Rotate<Self> {
    Rotate {
      cube: self,
      axis,
      amount,
    }
  }
}

impl<T: Cube> MakeRotate for T {}
