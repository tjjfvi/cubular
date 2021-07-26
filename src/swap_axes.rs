use crate::*;

pub struct SwapAxes<C: Cube> {
  cube: C,
  from: Axis,
  to: Axis,
}

impl<C: Cube> Cube for SwapAxes<C> {
  fn get(&self, pos: Pos) -> crate::n::N {
    self.cube.get(pos.swap_axes(self.from, self.to))
  }
  unsafe fn set(&mut self, pos: Pos, val: N) {
    self.cube.set(pos.swap_axes(self.from, self.to), val)
  }
  fn size(&self) -> Pos {
    self.cube.size().swap_axes(self.from, self.to)
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
