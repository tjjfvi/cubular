use crate::*;
use lazy_static::lazy_static;
use tap::Tap;

lazy_static! {
  static ref INTO_CENTER: Vec<Move> = vec![
    Move(Pos(1, 2, 2), Axis::Z, 1),
    Move(Pos(2, 3, 2), Axis::Z, -1),
    Move(Pos(1, 2, 2), Axis::Z, -1),
    Move(Pos(2, 3, 2), Axis::Z, 1),
  ];
  static ref INTO_CENTER_REVERSE: Vec<Move> = INTO_CENTER.clone().reverse_moves();
}

pub struct SolveInnerShell;
impl SolveStep for SolveInnerShell {
  fn move_pool<C: Cube>(&self, cube: &mut C, from: Pos, to: Pos) {
    let center = Pos(2, 2, 2);
    if from.parity() != to.parity() {
      panic!("Cannot move between positions of different parities");
    }
    if from == to {
      return;
    }
    if from.parity() == 1 {
      fn in_center(pos: Pos) -> bool {
        ((pos.0 == 2) as u8 + (pos.1 == 2) as u8 + (pos.2 == 2) as u8) == 2
      }
      if in_center(from) {
        if in_center(to) {
          fn info(pos: Pos) -> (Axis, i8) {
            let axis = if pos.0 != 2 {
              Axis::X
            } else if pos.1 != 2 {
              Axis::Y
            } else {
              Axis::Z
            };
            (axis, if pos[axis] == 3 { 1 } else { -1 })
          }
          let (from_axis, from_dir) = info(from);
          let (to_axis, to_dir) = info(to);
          if from_axis == to_axis {
            cube.apply_move(Move(
              center,
              if to_axis == Axis::X { Axis::Y } else { Axis::X },
              2,
            ))
          } else {
            let (rot_axis, rot_dir) = match (from_axis, to_axis) {
              (Axis::X, Axis::Y) => (Axis::Z, 1),
              (Axis::Y, Axis::X) => (Axis::Z, -1),
              (Axis::X, Axis::Z) => (Axis::Y, 1),
              (Axis::Z, Axis::X) => (Axis::Y, -1),
              (Axis::Y, Axis::Z) => (Axis::X, 1),
              (Axis::Z, Axis::Y) => (Axis::X, -1),
              _ => panic!("Unreachable"),
            };
            cube.apply_move(Move(center, rot_axis, from_dir * to_dir * rot_dir));
          }
        } else {
          self.move_pool(cube, from, Pos(2, 2, 1));
          cube.apply_moves(INTO_CENTER_REVERSE.clone());
          self.move_pool(cube, Pos(1, 3, 1), to);
        }
      } else {
        if in_center(to) {
          self.move_pool(cube, from, Pos(1, 3, 1));
          cube.apply_moves(INTO_CENTER.clone());
          self.move_pool(cube, Pos(2, 2, 1), to);
        } else {
          fn f(pos: Pos) -> i8 {
            if pos.0 == 1 {
              if pos.1 == 1 {
                0
              } else {
                1
              }
            } else {
              if pos.1 == 1 {
                3
              } else {
                2
              }
            }
          }
          cube.apply_move(Move(center, Axis::Z, f(from) - f(to)));
          if to.2 != from.2 {
            cube.apply_move(Move(center, Axis::Y, (to.0 + to.2 - 1) as i8 % 4));
          }
        }
      }
    } else {
      if from == center {
        cube.apply_moves(INTO_CENTER_REVERSE.clone());
        self.move_pool(cube, Pos(1, 3, 2), to);
      } else if to == center {
        self.move_pool(cube, from, Pos(1, 3, 2));
        cube.apply_moves(INTO_CENTER.clone());
      } else if from.2 != to.2 {
        let mut from = from;
        while from.2 != to.2 {
          let axis = if from.0 == 2 { Axis::Y } else { Axis::X };
          cube.apply_move(Move(center, axis, 1));
          from = from.rotate(axis, 1, 5);
        }
        self.move_pool(cube, from, to);
      } else {
        fn f(pos: Pos) -> i8 {
          if pos.0 <= 2 && pos.1 == 1 {
            3
          } else if pos.0 == 3 && pos.1 <= 2 {
            2
          } else if pos.0 >= 2 && pos.1 == 3 {
            1
          } else {
            0
          }
        }
        cube.apply_move(Move(center, Axis::Z, f(from) - f(to)));
      }
    }
  }
  fn classify<C: Cube>(&self, cube: &C, pos: Pos) -> PosClass {
    match pos {
      // move the center into position at the end
      Pos(2, 2, 2) => PosClass::Active {
        index: 100,
        source: Pos(2, 2, 2),
        moves: vec![],
      },
      p if in_inner(p) => PosClass::Pool,
      _ => match pos {
        p if p.0 >= 3 || p.1 >= 3 => self
          .classify(cube, pos.rotate(Axis::Z, 1, 5))
          .tap_mut(|x| x.rotate(Axis::Z, -1, 5)),
        p if p.2 >= 1 => {
          let axis = if p.1 == 0 { Axis::X } else { Axis::Y };
          self
            .classify(cube, pos.rotate(axis, 1, 5))
            .tap_mut(|x| x.rotate(axis, -1, 5))
        }
        p if p.1 > p.0 => self
          .classify(cube, pos.swap_axes(Axis::X, Axis::Y))
          .tap_mut(|x| x.swap_axes(Axis::X, Axis::Y)),
        Pos(0, 0, 0) => PosClass::Active {
          index: 0,
          source: Pos(2, 2, 2),
          moves: vec![
            Move(Pos(1, 1, 1), Axis::Z, 2),
            Move(Pos(1, 1, 1), Axis::X, 1),
          ],
        },
        Pos(1, 0, 0) => PosClass::Active {
          index: 1,
          source: Pos(2, 3, 2),
          moves: vec![
            Move(Pos(1, 1, 1), Axis::Z, 1),
            Move(Pos(1, 1, 1), Axis::Y, 1),
            Move(Pos(2, 2, 2), Axis::Z, 2),
            Move(Pos(1, 1, 1), Axis::Y, -1),
            Move(Pos(1, 1, 1), Axis::Z, -1),
          ],
        },
        Pos(2, 0, 0) => PosClass::Active {
          index: 2,
          source: Pos(2, 2, 2),
          moves: vec![
            Move(Pos(2, 3, 2), Axis::Z, -1),
            Move(Pos(1, 1, 1), Axis::Z, 1),
            Move(Pos(1, 1, 1), Axis::Y, 1),
            Move(Pos(2, 3, 2), Axis::Z, 1),
            Move(Pos(1, 1, 1), Axis::Y, -1),
            Move(Pos(1, 1, 1), Axis::Z, -1),
          ],
        },
        Pos(1, 1, 0) => PosClass::Active {
          index: 3,
          source: Pos(3, 3, 2),
          moves: vec![
            Move(Pos(1, 1, 1), Axis::Z, 1),
            Move(Pos(1, 1, 1), Axis::Y, 2),
            Move(Pos(2, 2, 2), Axis::Z, 2),
            Move(Pos(1, 1, 1), Axis::Y, 2),
            Move(Pos(1, 1, 1), Axis::Z, -1),
          ],
        },
        Pos(2, 1, 0) => PosClass::Active {
          index: 4,
          source: Pos(1, 2, 2),
          moves: vec![
            Move(Pos(2, 3, 2), Axis::Z, -1),
            Move(Pos(1, 1, 1), Axis::Z, -1),
            Move(Pos(1, 1, 1), Axis::X, 2),
            Move(Pos(2, 3, 2), Axis::Z, 1),
            Move(Pos(1, 1, 1), Axis::X, 2),
            Move(Pos(1, 1, 1), Axis::Z, 1),
          ],
        },
        Pos(2, 2, 0) => PosClass::Active {
          index: 5,
          source: Pos(2, 3, 3),
          moves: vec![
            Move(Pos(1, 1, 1), Axis::Z, 1),
            Move(Pos(1, 1, 1), Axis::Y, 2),
            Move(Pos(3, 3, 2), Axis::X, 1),
            Move(Pos(1, 1, 1), Axis::Y, 2),
            Move(Pos(1, 1, 1), Axis::Z, -1),
            Move(Pos(3, 3, 2), Axis::X, -1),
          ],
        },
        _ => panic!("Unreachable"),
      },
    }
  }
}

fn in_inner(p: Pos) -> bool {
  p.0 >= 1 && p.0 <= 3 && p.1 >= 1 && p.1 <= 3 && p.2 >= 1 && p.2 <= 3
}
