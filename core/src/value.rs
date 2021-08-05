use std::{
  fmt::{self, Debug, Write},
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ValueCharset {
  Alpha,
  ZeroModNine,
  OneModNine,
}

static CHARSET_ALPHA: [char; 18] = [
  '0', 'a', '1', 'b', '2', 'c', '3', 'd', '4', 'e', '5', 'f', '6', 'g', '7', 'h', '8', 'i',
];

static CHARSET_ZERO_MOD_NINE: [char; 18] = [
  '0', '1', '2', '3', '4', '5', '6', '7', '8', '0', '1', '2', '3', '4', '5', '6', '7', '8',
];

static CHARSET_ONE_MOD_NINE: [char; 18] = [
  '1', '2', '3', '4', '5', '6', '7', '8', '9', '1', '2', '3', '4', '5', '6', '7', '8', '9',
];

impl Value {
  pub fn to_char(&self, charset: ValueCharset) -> char {
    (match charset {
      ValueCharset::Alpha => CHARSET_ALPHA,
      ValueCharset::ZeroModNine => CHARSET_ZERO_MOD_NINE,
      ValueCharset::OneModNine => CHARSET_ONE_MOD_NINE,
    })[self.0]
  }
}

impl Debug for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
    f.write_str("N(")?;
    f.write_char(CHARSET_ALPHA[self.0])?;
    f.write_str(")")?;
    Ok(())
  }
}
