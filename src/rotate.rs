use crate::*;

pub trait Rotate<'a>: Cube + Sized + 'a {
  fn rotate_x(self) -> Flip<SwapAxes<Self>> {
    self.swap_axes(Axis::Y, Axis::Z).flip(Axis::Y)
  }
  fn rotate_y(self) -> Flip<SwapAxes<Self>> {
    self.swap_axes(Axis::X, Axis::Z).flip(Axis::X)
  }
  fn rotate_z(self) -> Flip<SwapAxes<Self>> {
    self.swap_axes(Axis::X, Axis::Y).flip(Axis::X)
  }
  fn rotate_once(self, axis: Axis) -> Flip<SwapAxes<Self>> {
    match axis {
      Axis::X => self.rotate_x(),
      Axis::Y => self.rotate_y(),
      Axis::Z => self.rotate_z(),
    }
  }
  fn rotate(self, axis: Axis, amount: i8) -> Box<dyn Cube + 'a> {
    let mut cur: Box<dyn Cube> = Box::new(self);
    for _ in 0..(amount.rem_euclid(4)) {
      cur = Box::new(cur.rotate_once(axis))
    }
    cur
  }
}

impl<'a, T: Cube + 'a> Rotate<'a> for T {}
