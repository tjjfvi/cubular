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
  fn get_solved(&self, pos: Pos) -> crate::n::N {
    self.cube.get_solved(self.transform_pos(pos))
  }
  fn apply_move(&mut self, mut m: Move) {
    m.0 = self.transform_pos(m.0);
    if m.1 != self.axis {
      m.2 = -m.2;
    }
    self.cube.apply_move(m);
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
