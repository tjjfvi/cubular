mod solve0;
mod solve1;
mod solve2;
mod swap;

pub(self) use crate::*;
pub(self) use solve0::*;
pub(self) use solve1::*;
pub(self) use solve2::*;
pub(self) use swap::*;

pub trait Solve: Cube {
  fn solve(&self) {
    self.solve0();
    self.solve1();
    self.solve2();
  }
}

impl<T: Cube> Solve for T {}
