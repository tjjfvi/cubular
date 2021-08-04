use std::{
  fmt::{self, Debug, Display, Write},
  ops::{Add, Sub},
};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Default)]
pub struct Value(pub usize);

impl From<usize> for Value {
  #[inline]
  fn from(x: usize) -> Self {
    Value(x % 18)
  }
}

impl Add<Value> for Value {
  type Output = Value;
  fn add(self, rhs: Value) -> Self::Output {
    Value((self.0 + rhs.0) % 18)
  }
}

impl Sub<Value> for Value {
  type Output = Value;
  fn sub(self, rhs: Value) -> Self::Output {
    Value((18 + self.0 - rhs.0) % 18)
  }
}

static CHARS: [char; 18] = [
  // '0', 'b', '2', 'd', '4', 'f', '6', 'h', '8', 'a', '1', 'c', '3', 'e', '5', 'g', '6', 'i',
  '0', 'a', '1', 'b', '2', 'c', '3', 'd', '4', 'e', '5', 'f', '6', 'g', '7', 'h', '8', 'i',
];

impl Display for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
    f.write_char(CHARS[self.0])?;
    Ok(())
  }
}

impl Debug for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
    f.write_str("N(")?;
    f.write_char(CHARS[self.0])?;
    f.write_str(")")?;
    Ok(())
  }
}
