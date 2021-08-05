use crate::*;

pub fn get_move_str(Move(Pos(x, y, z), axis, amount): &Move) -> String {
  format!(
    "{}{}{}{}{}",
    x,
    y,
    z,
    match axis {
      Axis::X => "X",
      Axis::Y => "Z",
      Axis::Z => "Y",
    },
    (amount * if *axis == Axis::X { 1 } else { -1 })
      .rem_euclid(4)
      .to_string()
  )
}

pub fn get_moves_str(moves: &Vec<Move>) -> String {
  moves
    .iter()
    .map(get_move_str)
    .collect::<Vec<_>>()
    .join("\n")
}

pub fn parse_move_str(str: &str) -> Result<Move, String> {
  let str = str.trim();
  if str.len() != 5 {
    return Err(format!("Invalid move: expected length of exactly 5"));
  }
  let parse_coord = |i| {
    str[i..=i]
      .parse::<usize>()
      .map_err(|_| format!("Invalid coordinate: must be a number"))
      .and_then(|x| {
        if x >= 1 && x <= 7 {
          Ok(x)
        } else {
          Err(format!("Invalid coordinate: must be in 1..=7"))
        }
      })
  };
  let center = Pos(parse_coord(0)?, parse_coord(1)?, parse_coord(2)?);
  let axis = match str.chars().nth(3) {
    Some('X') => Axis::X,
    Some('Z') => Axis::Y,
    Some('Y') => Axis::Z,
    _ => return Err(format!("Invalid axis: must be X, Y, or Z")),
  };
  let amount = str[4..=4]
    .parse::<i8>()
    .map_err(|_| format!("Invalid move count: must be a number"))?;
  Ok(Move(center, axis, amount))
}

pub fn parse_moves_str(str: &str) -> Result<Vec<Move>, String> {
  str
    .split("\n")
    .map(str::trim)
    .filter(|x| *x != "")
    .map(parse_move_str)
    .collect()
}
