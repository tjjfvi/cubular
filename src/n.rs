use std::{
  fmt::{self, Debug, Display, Write},
  ops::{Add, Sub},
};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct N(pub usize);

impl From<usize> for N {
  #[inline]
  fn from(x: usize) -> Self {
    N(x % 18)
  }
}

impl Add<N> for N {
  type Output = N;
  fn add(self, rhs: N) -> Self::Output {
    N((self.0 + rhs.0) % 18)
  }
}

impl Sub<N> for N {
  type Output = N;
  fn sub(self, rhs: N) -> Self::Output {
    N((18 + self.0 - rhs.0) % 18)
  }
}

static CHARS: [char; 18] = [
  // '0', 'b', '2', 'd', '4', 'f', '6', 'h', '8', 'a', '1', 'c', '3', 'e', '5', 'g', '6', 'i',
  '0', 'a', '1', 'b', '2', 'c', '3', 'd', '4', 'e', '5', 'f', '6', 'g', '7', 'h', '8', 'i',
];

impl Display for N {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
    f.write_char(CHARS[self.0])?;
    Ok(())
  }
}

impl Debug for N {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
    f.write_str("N(")?;
    f.write_char(CHARS[self.0])?;
    f.write_str(")")?;
    Ok(())
  }
}
