use crate::*;

pub trait MoveValue: Cube + Sized {
  fn move_value(&self, value: N, to: Pos) -> Option<()> {
    if self.get(to) == value {
      return Some(());
    }
    let (from, _) = self.iter().find(|(_, n)| *n == value)?;
    self.move_piece(from, to);
    Some(())
  }
}

impl<T: Cube> MoveValue for T {}
