use crate::*;

pub struct Slice<C: Cube> {
  pub cube: C,
  pub min: Pos,
  pub size: Pos,
}

impl<C: Cube> Cube for Slice<C> {
  fn get(&self, pos: Pos) -> crate::n::N {
    self.cube.get(pos + self.min)
  }
  fn get_solved(&self, pos: Pos) -> crate::n::N {
    self.cube.get_solved(pos + self.min)
  }
  fn apply_move(&mut self, mut m: Move) {
    m.0 = m.0 + self.min;
    self.cube.apply_move(m)
  }
  fn size(&self) -> Pos {
    self.size
  }
}

pub trait MakeSlice: Cube + Sized {
  fn slice(self, min: Pos, size: Pos) -> Slice<Self> {
    Slice {
      cube: self,
      min,
      size,
    }
  }
}

impl<T: Cube> MakeSlice for T {}
