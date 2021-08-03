use crate::*;

#[derive(Debug)]
pub struct Swap {
  pub source: Pos,
  pub moves: Vec<Move>,
}

impl Swap {
  pub fn swap_axes(&mut self, from: Axis, to: Axis) {
    self.source = self.source.swap_axes(from, to);
    for m in self.moves.iter_mut() {
      m.0 = m.0.swap_axes(from, to);
      if m.1 == from {
        m.1 = to;
      } else if m.1 == to {
        m.1 = from;
      } else {
        m.1;
        m.2 = -m.2;
      }
    }
  }
  pub fn rotate(&mut self, axis: Axis, amount: i8, max: usize) {
    let amount = amount.rem_euclid(4);
    if amount == 0 {
      return;
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
          Axis::Y => Axis::Z,
          Axis::Z => Axis::X,
        }
      {
        m.2 = -m.2;
      }
    }
  }
}
