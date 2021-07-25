use crate::*;

#[derive(Debug)]
pub struct RootCube([[[N; 9]; 9]; 9]);

impl RootCube {
  #[must_use]
  pub fn solved() -> RootCube {
    #[inline(never)] // Prevent huge compiled sizes
    fn x9<F: Fn((usize, I)) -> T, I: Copy, T>(cb: F) -> impl Fn(I) -> [T; 9] {
      move |i| {
        [
          cb((0, i)),
          cb((1, i)),
          cb((2, i)),
          cb((3, i)),
          cb((4, i)),
          cb((5, i)),
          cb((6, i)),
          cb((7, i)),
          cb((8, i)),
        ]
      }
    }

    RootCube(x9(x9(x9(|(x, (y, (z, _)))| N((x + y + z) % 9))))(()))
  }
}

impl Cube for RootCube {
  fn get(&self, pos: Pos) -> N {
    self.0[pos.0][pos.1][pos.2]
  }
  fn size(&self) -> Pos {
    Pos(9, 9, 9)
  }
}
