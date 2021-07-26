use crate::*;

pub struct Flip<C: Cube> {
  cube: C,
  axis: Axis,
}

impl<C: Cube> Flip<C> {
  pub fn transform_pos(&self, pos: Pos) -> Pos {
    pos.flip(self.axis, self.size()[self.axis])
  }
}

impl<C: Cube> Cube for Flip<C> {
  fn get(&self, pos: Pos) -> crate::n::N {
    self.cube.get(self.transform_pos(pos))
  }
  unsafe fn set(&mut self, pos: Pos, val: N) {
    self.cube.set(self.transform_pos(pos), val)
  }
  fn size(&self) -> Pos {
    self.cube.size()
  }
}

pub trait MakeFlip: Cube + Sized {
  fn flip(self, axis: Axis) -> Flip<Self> {
    Flip { cube: self, axis }
  }
}

impl<T: Cube> MakeFlip for T {}
