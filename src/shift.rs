use crate::{cube::Cube, n::N, pos::Pos};

pub struct Shift<C: Cube> {
  pub cube: C,
  pub shift: N,
}

impl<C: Cube> Cube for Shift<C> {
  fn get(&self, pos: Pos) -> crate::n::N {
    self.cube.get(pos) + self.shift
  }
  fn size(&self) -> Pos {
    self.cube.size()
  }
}

pub trait MakeShift: Cube + Sized {
  fn shift(self, shift: N) -> Shift<Self> {
    Shift { cube: self, shift }
  }
}

impl<T: Cube> MakeShift for T {}
