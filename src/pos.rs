use std::ops::{Add, Index, IndexMut, Sub};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Pos(pub usize, pub usize, pub usize);

impl Pos {
  pub fn swap_axes(mut self, from: Axis, to: Axis) -> Pos {
    let f = self[from];
    self[from] = self[to];
    self[to] = f;
    self
  }
  pub fn flip(mut self, axis: Axis, max: usize) -> Pos {
    self[axis] = max - 1 - self[axis];
    self
  }
  pub fn rotate(self, axis: Axis, amount: i8, max: usize) -> Pos {
    let mut cur = self;
    for _ in 0..(amount.rem_euclid(4)) {
      cur = match axis {
        Axis::X => cur.swap_axes(Axis::Y, Axis::Z).flip(Axis::Y, max),
        Axis::Y => cur.swap_axes(Axis::X, Axis::Z).flip(Axis::X, max),
        Axis::Z => cur.swap_axes(Axis::X, Axis::Y).flip(Axis::X, max),
      };
    }
    cur
  }
  pub fn parity(self) -> usize {
    (self.0 + self.1 + self.2) % 2
  }
  pub fn within(self, min: Pos, max: Pos) -> bool {
    true
      && self.0 >= min.0
      && self.0 <= max.0
      && self.1 >= min.1
      && self.1 <= max.1
      && self.2 >= min.2
      && self.2 <= max.2
  }
}

impl Add for Pos {
  type Output = Pos;

  fn add(self, rhs: Self) -> Self::Output {
    Pos(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
  }
}

impl Sub for Pos {
  type Output = Pos;

  fn sub(self, rhs: Self) -> Self::Output {
    Pos(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
  }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Axis {
  X = 0,
  Y = 1,
  Z = 2,
}

impl Index<Axis> for Pos {
  type Output = usize;
  fn index(&self, index: Axis) -> &Self::Output {
    match index {
      Axis::X => &self.0,
      Axis::Y => &self.1,
      Axis::Z => &self.2,
    }
  }
}

impl IndexMut<Axis> for Pos {
  fn index_mut(&mut self, index: Axis) -> &mut Self::Output {
    match index {
      Axis::X => &mut self.0,
      Axis::Y => &mut self.1,
      Axis::Z => &mut self.2,
    }
  }
}
