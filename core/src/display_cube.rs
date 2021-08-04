use std::{cmp::min, fmt::Display};

use colored::Colorize;

use crate::*;

pub struct DisplayCube<'a, T: Cube + 'a>(pub &'a T);

impl<'a, T: Cube + 'a> Display for DisplayCube<'a, T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let size = self.0.size();
    let grid_group_size = (size.2 as f32).sqrt().ceil() as usize;
    for grid_group in 0..(size.2 + grid_group_size - 1) / grid_group_size {
      if grid_group != 0 {
        f.write_str("\n\n")?;
      }
      for row in 0..size.1 {
        if row != 0 {
          f.write_str("\n")?;
        }
        for grid in grid_group * grid_group_size..min((grid_group + 1) * grid_group_size, size.2) {
          if grid != grid_group * grid_group_size {
            f.write_str("  ")?;
          }
          for col in 0..size.0 {
            if col != 0 {
              f.write_str(" ")?;
            }
            let pos = Pos(col, row, grid);
            let mut str = self.0.get(pos).to_string();
            if self.0.get(pos) == self.0.get_solved(pos) {
              str = str.green().to_string();
            }
            f.write_str(&str[..])?;
          }
        }
      }
    }
    Ok(())
  }
}

impl<'a, T: Cube + 'a> From<&'a T> for DisplayCube<'a, T> {
  fn from(arg: &'a T) -> Self {
    DisplayCube(arg)
  }
}

pub trait PrintCube: Cube + Sized {
  fn print(&self) {
    println!("\n{}\n", DisplayCube(self));
  }
}

impl<T: Cube> PrintCube for T {}
