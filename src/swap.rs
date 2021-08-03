use crate::*;

#[derive(Debug)]
pub struct Swap {
  pub source: Pos,
  pub moves: Vec<Move>,
}

impl Swap {
  pub fn rotate(mut self, axis: Axis, amount: i8, max: usize) -> Self {
    let amount = amount.rem_euclid(4);
    if amount == 0 {
      return self;
    }
    self.source = self.source.rotate(axis, amount, max);
    for m in self.moves.iter_mut() {
      m.0 = m.0.rotate(axis, amount, max);
      if amount != 2 {
        m.1 = match (axis, m.1) {
          (Axis::Y, Axis::Z) | (Axis::Z, Axis::Y) => Axis::X,
          (Axis::X, Axis::Z) | (Axis::Z, Axis::X) => Axis::Y,
          (Axis::X, Axis::Y) | (Axis::Y, Axis::X) => Axis::Z,
          _ => axis,
        };
      }
      if m.1
        == match axis {
          Axis::X => Axis::Y,
          Axis::Y => Axis::X,
          Axis::Z => Axis::X,
        }
      {
        m.2 = -m.2;
      }
    }
    self
  }
}
