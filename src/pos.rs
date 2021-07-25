use std::ops::{Add, Index, IndexMut, Sub};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos(pub usize, pub usize, pub usize);

impl Add for Pos {
  type Output = Pos;

  fn add(self, rhs: Self) -> Self::Output {
    Pos(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
  }
}

impl Sub for Pos {
  type Output = Pos;

  fn sub(self, rhs: Self) -> Self::Output {
    Pos(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
  }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Axis {
  X = 0,
  Y = 1,
  Z = 2,
}

impl Index<Axis> for Pos {
  type Output = usize;
  fn index(&self, index: Axis) -> &Self::Output {
    match index {
      Axis::X => &self.0,
      Axis::Y => &self.1,
      Axis::Z => &self.2,
    }
  }
}

impl IndexMut<Axis> for Pos {
  fn index_mut(&mut self, index: Axis) -> &mut Self::Output {
    match index {
      Axis::X => &mut self.0,
      Axis::Y => &mut self.1,
      Axis::Z => &mut self.2,
    }
  }
}
