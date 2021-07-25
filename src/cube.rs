use std::{cmp::min, fmt::Display};

use crate::{n::N, pos::Pos};

pub trait Cube {
  fn get(&self, pos: Pos) -> N;
  // fn set(&mut self, pos: Pos, val: N);
  fn size(&self) -> Pos;
}

pub struct DisplayCube<T: Cube>(T);

impl<T: Cube> Display for DisplayCube<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let size = self.0.size();
    for grid_group in 0..(size.2 + 2) / 3 {
      if grid_group != 0 {
        f.write_str("\n\n")?;
      }
      for row in 0..size.1 {
        if row != 0 {
          f.write_str("\n")?;
        }
        for grid in grid_group * 3..min(grid_group * 3 + 3, size.2) {
          if grid != grid_group * 3 {
            f.write_str("  ")?;
          }
          for col in 0..size.0 {
            if col != 0 {
              f.write_str(" ")?;
            }
            let pos = Pos(col, row, grid);
            <usize as Display>::fmt(&self.0.get(pos).0, f)?;
          }
        }
      }
    }
    Ok(())
  }
}

impl<T: Cube> From<T> for DisplayCube<T> {
  fn from(arg: T) -> Self {
    DisplayCube(arg)
  }
}
