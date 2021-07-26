use std::cell::UnsafeCell;

use crate::*;

#[derive(Debug)]
pub struct RootCube(UnsafeCell<[[[N; 9]; 9]; 9]>);

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

    RootCube(UnsafeCell::new(x9(x9(x9(|(x, (y, (z, _)))| {
      N((x + y + z) % 18)
    })))(())))
  }
}

impl Cube for RootCube {
  fn get(&self, pos: Pos) -> N {
    unsafe { (*self.0.get())[pos.0][pos.1][pos.2] }
  }
  unsafe fn set(&mut self, pos: Pos, val: N) {
    (*self.0.get())[pos.0][pos.1][pos.2] = val
  }
  fn size(&self) -> Pos {
    Pos(9, 9, 9)
  }
}
