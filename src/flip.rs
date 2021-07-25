use crate::*;

pub struct Flip<C: Cube> {
  cube: C,
  axis: Axis,
}

impl<C: Cube> Flip<C> {
  pub fn transform_pos(&self, mut pos: Pos) -> Pos {
    pos[self.axis] = self.size()[self.axis] - 1 - pos[self.axis];
    pos
  }
}

impl<C: Cube> Cube for Flip<C> {
  fn get(&self, pos: Pos) -> crate::n::N {
    self.cube.get(self.transform_pos(pos))
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
