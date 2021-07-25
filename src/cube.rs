use crate::*;

pub trait Cube {
  fn get(&self, pos: Pos) -> N;
  // fn set(&mut self, pos: Pos, val: N);
  fn size(&self) -> Pos;
}

impl Cube for Box<dyn Cube> {
  fn get(&self, pos: Pos) -> N {
    (**self).get(pos)
  }
  fn size(&self) -> Pos {
    (**self).size()
  }
}
