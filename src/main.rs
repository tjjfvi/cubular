use crate::{cube::FullCube, pos::Pos};

mod cube;
mod n;
mod pos;
mod transform;

fn main() {
  println!(
    "{}",
    FullCube::solved().slice().slice(Pos(1, 2, 3), Pos(3, 3, 3))
  );
}
