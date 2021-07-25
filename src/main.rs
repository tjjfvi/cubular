use crate::{cube::DisplayCube, root_cube::RootCube};

mod cube;
mod n;
mod pos;
mod root_cube;
mod shift;
mod slice;

fn main() {
  println!("{}", DisplayCube::from(RootCube::solved()));
}
