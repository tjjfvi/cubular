use std::{
  cmp::min,
  fmt::{Debug, Display},
};

use crate::{
  n::N,
  pos::Pos,
  transform::{Transform, Translate},
};

#[derive(Debug)]
pub struct FullCube([[[N; 9]; 9]; 9]);

impl FullCube {
  #[must_use]
  pub fn solved() -> FullCube {
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

    FullCube(x9(x9(x9(|(x, (y, (z, _)))| N((x + y + z) % 9))))(()))
  }
  pub fn slice<'a>(&'a self) -> CubeSlice<'a, ()> {
    CubeSlice {
      full_cube: self,
      transform: (),
      size: Pos(9, 9, 9),
      shift: N(0),
    }
  }
  pub fn get(&self, pos: Pos) -> N {
    self.0[pos.0][pos.1][pos.2]
  }
}

pub struct CubeSlice<'a, T: Transform> {
  pub full_cube: &'a FullCube,
  pub transform: T,
  pub size: Pos,
  pub shift: N,
}

impl<'a, T: Transform> CubeSlice<'a, T> {
  pub fn slice(&self, min: Pos, size: Pos) -> CubeSlice<'a, (Translate, T)> {
    CubeSlice {
      full_cube: self.full_cube,
      transform: (Translate(min), self.transform.clone()),
      size,
      shift: self.shift,
    }
  }
  pub fn shift(&self, shift: N) -> CubeSlice<'a, T> {
    CubeSlice {
      full_cube: self.full_cube,
      transform: self.transform.clone(),
      size: self.size,
      shift: self.shift + shift,
    }
  }
  pub fn get(&self, pos: Pos) -> N {
    self.full_cube.get(self.transform.transform(pos)) + self.shift
  }
}

impl<'a, T: Transform> Display for CubeSlice<'a, T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for grid_group in 0..(self.size.2 + 2) / 3 {
      if grid_group != 0 {
        f.write_str("\n\n")?;
      }
      for row in 0..self.size.1 {
        if row != 0 {
          f.write_str("\n")?;
        }
        for grid in grid_group * 3..min(grid_group * 3 + 3, self.size.2) {
          if grid != grid_group * 3 {
            f.write_str("  ")?;
          }
          for col in 0..self.size.0 {
            if col != 0 {
              f.write_str(" ")?;
            }
            let pos = Pos(col, row, grid);
            <usize as Display>::fmt(&self.get(pos).0, f)?;
          }
        }
      }
    }
    Ok(())
  }
}
