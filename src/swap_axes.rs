use crate::*;

pub struct SwapAxes<C: Cube> {
  cube: C,
  from: Axis,
  to: Axis,
}

impl<C: Cube> SwapAxes<C> {
  pub fn transform_pos(&self, mut pos: Pos) -> Pos {
    let from = pos[self.from];
    pos[self.from] = pos[self.to];
    pos[self.to] = from;
    pos
  }
}

impl<C: Cube> Cube for SwapAxes<C> {
  fn get(&self, pos: Pos) -> crate::n::N {
    self.cube.get(self.transform_pos(pos))
  }
  fn set(&mut self, pos: Pos, val: N) {
    self.cube.set(self.transform_pos(pos), val)
  }
  fn size(&self) -> Pos {
    self.transform_pos(self.cube.size())
  }
}

pub trait MakeSwapAxes: Cube + Sized {
  fn swap_axes(self, from: Axis, to: Axis) -> SwapAxes<Self> {
    SwapAxes {
      cube: self,
      from,
      to,
    }
  }
}

impl<T: Cube> MakeSwapAxes for T {}
