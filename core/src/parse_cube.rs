use hashbag::HashBag;
use lazy_static::lazy_static;
use regex::Regex;

use crate::*;

pub fn parse_cube(cube: &mut RootCube, str: &str) -> Result<(), String> {
  let str = str.to_lowercase();
  let str = str.trim();
  let charset = if str.contains('a') {
    "0a1b2c3d4e5f6g7h8i"
  } else if str.contains('0') {
    "012345678"
  } else {
    "123456789"
  };
  let mut z = 0;
  lazy_static! {
    static ref PARAGRAPH_SEP: Regex = Regex::new(r"\s*\n\s*\n\s*").unwrap();
  }
  for paragraph in PARAGRAPH_SEP.split(str) {
    let paragraph = paragraph.trim();
    let mut z2 = z;
    for (y, line) in paragraph.split("\n").enumerate() {
      let line = line.trim();
      z2 = z;
      let row_sep = if line.chars().nth(1) == Some(' ') {
        "  "
      } else {
        " "
      };
      for row in line.split(row_sep).filter(|x| *x != "") {
        let row = row.trim();
        for (x, char) in row.chars().filter(|x| *x != ' ').enumerate() {
          if x >= 9 || y >= 9 || z2 >= 9 {
            dbg!((x, y, z2));
            return Err(format!("Invalid cube size {:?}", (x, y, z2)));
          }
          let mut value = if let Some(index) = charset.find(char) {
            index
          } else {
            return Err(format!("Invalid character {:?}", char));
          };
          if value % 2 != (x + y + z2) % 2 {
            if charset.len() == 9 {
              value += 9
            } else {
              return Err(format!("Invalid parity at position {:?}", (x, y, z2)));
            }
          };
          cube.values[x][y][z2].0 = value;
        }
        z2 += 1;
      }
    }
    z = z2;
  }
  if cube
    .values
    .iter()
    .flatten()
    .flatten()
    .collect::<HashBag<_>>()
    != SOLVED.iter().flatten().flatten().collect()
  {
    return Err(format!("Invalid piece counts"));
  }
  Ok(())
}
