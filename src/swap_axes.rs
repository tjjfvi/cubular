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
  fn get_solved(&self, pos: Pos) -> crate::n::N {
    self.cube.get_solved(pos.swap_axes(self.from, self.to))
  }
  fn apply_move(&mut self, mut m: Move) {
    if self.from != self.to {
      m.0 = m.0.swap_axes(self.from, self.to);
      match (self.from, self.to) {
        (Axis::X, Axis::Z) | (Axis::Z, Axis::X) if m.1 != Axis::Y => m.2 = -m.2,
        _ => {}
      }
      if m.1 == self.from {
        m.1 = self.to;
      } else if m.1 == self.to {
        m.1 = self.from;
      } else {
        m.2 = -m.2;
      }
    }
    self.cube.apply_move(m)
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
