use crate::*;

pub struct CubeIter<'a, C: Cube> {
  cube: &'a C,
  pos: Pos,
}

impl<'a, C: Cube> Iterator for CubeIter<'a, C> {
  type Item = (Pos, N);
  fn next(&mut self) -> Option<Self::Item> {
    let size = self.cube.size();
    self.pos.0 += 1;
    if self.pos.0 >= size.0 {
      self.pos.0 = 0;
      self.pos.1 += 1;
      if self.pos.1 >= size.1 {
        self.pos.1 = 0;
        self.pos.2 += 1;
        if self.pos.2 >= size.2 {
          self.pos.2 = size.2;
          return None;
        }
      }
    }
    Some((self.pos, self.cube.get(self.pos)))
  }
  fn size_hint(&self) -> (usize, Option<usize>) {
    let size = self.cube.size();
    let items_left =
      size.0 * size.1 * size.2 - self.pos.0 - self.pos.1 * size.0 - self.pos.2 * size.0 * size.1;
    (items_left, Some(items_left))
  }
}

pub trait MakeCubeIter: Cube + Sized {
  fn iter<'a>(&'a self) -> CubeIter<'a, Self> {
    CubeIter {
      cube: self,
      pos: Pos(0, 0, 0),
    }
  }
}

impl<T: Cube> MakeCubeIter for T {}
