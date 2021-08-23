use std::fmt::{self, Debug, Write};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Default)]
pub struct Value(pub u8);

static CHARSET: [char; 18] = [
  '0', 'a', '1', 'b', '2', 'c', '3', 'd', '4', 'e', '5', 'f', '6', 'g', '7', 'h', '8', 'i',
];

impl Value {
  pub fn to_char(&self) -> char {
    CHARSET[self.0 as usize]
  }
}

impl Debug for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
    f.write_str("N(")?;
    f.write_char(CHARSET[self.0 as usize])?;
    f.write_str(")")?;
    Ok(())
  }
}
