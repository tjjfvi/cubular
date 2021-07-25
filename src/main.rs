mod cube;
mod display_cube;
mod n;
mod pos;
mod root_cube;
mod shift;
mod slice;

pub use cube::*;
pub use display_cube::*;
pub use n::*;
pub use pos::*;
pub use root_cube::*;
pub use shift::*;
pub use slice::*;

fn main() {
  println!(
    "{}",
    DisplayCube::from(
      RootCube::solved()
        .slice(Pos(5, 0, 0), Pos(4, 4, 4))
        .shift(N(4))
    )
  );
}
