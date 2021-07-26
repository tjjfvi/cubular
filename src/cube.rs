use crate::*;

pub trait Cube {
  fn get(&self, pos: Pos) -> N;
  fn get_solved(&self, pos: Pos) -> N;
  unsafe fn set(&mut self, pos: Pos, val: N);
  fn size(&self) -> Pos;
}

impl<T: Cube + ?Sized> Cube for Box<T> {
  fn get(&self, pos: Pos) -> N {
    (**self).get(pos)
  }
  fn get_solved(&self, pos: Pos) -> N {
    (**self).get_solved(pos)
  }
  unsafe fn set(&mut self, pos: Pos, val: N) {
    (**self).set(pos, val)
  }
  fn size(&self) -> Pos {
    (**self).size()
  }
}

impl<T: Cube + ?Sized> Cube for &mut T {
  fn get(&self, pos: Pos) -> N {
    (**self).get(pos)
  }
  fn get_solved(&self, pos: Pos) -> N {
    (**self).get(pos)
  }
  unsafe fn set(&mut self, pos: Pos, val: N) {
    (**self).set(pos, val)
  }
  fn size(&self) -> Pos {
    (**self).size()
  }
}
