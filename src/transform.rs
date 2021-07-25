use crate::pos::Pos;

pub trait Transform: Clone {
  fn transform(&self, pos: Pos) -> Pos;
  fn untransform(&self, pos: Pos) -> Pos;
}

#[derive(Clone, Copy)]
pub struct Translate(pub Pos);

impl Transform for Translate {
  fn transform(&self, pos: Pos) -> Pos {
    pos + self.0
  }
  fn untransform(&self, pos: Pos) -> Pos {
    pos - self.0
  }
}

impl Transform for () {
  fn transform(&self, pos: Pos) -> Pos {
    pos
  }
  fn untransform(&self, pos: Pos) -> Pos {
    pos
  }
}

impl<T: Transform, U: Transform> Transform for (T, U) {
  fn transform(&self, pos: Pos) -> Pos {
    self.0.transform(self.1.transform(pos))
  }
  fn untransform(&self, pos: Pos) -> Pos {
    self.1.untransform(self.0.untransform(pos))
  }
}
