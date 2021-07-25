use std::{
  fmt::Debug,
  ops::{Add, Sub},
};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct N(pub usize);

impl From<usize> for N {
  #[inline]
  fn from(x: usize) -> Self {
    N(x % 9)
  }
}

impl Add<N> for N {
  type Output = N;
  fn add(self, rhs: N) -> Self::Output {
    N((self.0 + rhs.0) % 9)
  }
}

impl Sub<N> for N {
  type Output = N;
  fn sub(self, rhs: N) -> Self::Output {
    N((9 + self.0 - rhs.0) % 9)
  }
}
