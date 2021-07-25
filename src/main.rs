mod cube;
mod display_cube;
mod flip;
mod n;
mod pos;
mod root_cube;
mod shift;
mod slice;
mod swap_axes;

pub use cube::*;
pub use display_cube::*;
pub use flip::*;
pub use n::*;
pub use pos::*;
pub use root_cube::*;
pub use shift::*;
pub use slice::*;
pub use swap_axes::*;

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
