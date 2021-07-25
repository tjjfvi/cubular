use std::ops::{Add, Sub};

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
