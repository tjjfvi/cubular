use crate::*;
use lazy_static::lazy_static;
use rand::Rng;

pub trait SolveFinal: Cube + Sized {
  fn solve_final(&self) {
    self.slice(Pos(4, 4, 4), Pos(5, 5, 5)).shift(N(6))._solve()
  }
}

lazy_static! {
  static ref INTO_CENTER: Vec<Move> = vec![
    Move(Pos(1, 2, 2), Axis::Z, 1),
    Move(Pos(2, 3, 2), Axis::Z, -1),
    Move(Pos(1, 2, 2), Axis::Z, -1),
    Move(Pos(2, 3, 2), Axis::Z, 1),
  ];
  static ref INTO_CENTER_REVERSE: Vec<Move> = INTO_CENTER.clone().reverse_moves();
}

trait _SolveFinal: Cube + Sized {
  fn _solve(&self) {
    for _ in 0..10000 {
      println!("\n\n\n!!!\n\n\n");
      self.print();
      let mut rng = rand::thread_rng();
      let from = Pos(
        rng.gen_range(1..4),
        rng.gen_range(1..4),
        rng.gen_range(1..4),
      );
      let to = loop {
        let to = Pos(
          rng.gen_range(1..4),
          rng.gen_range(1..4),
          rng.gen_range(1..4),
        );
        if from.parity() == to.parity() {
          break to;
        }
      };
      let val = self.get(from);
      self._move_inner(from, to);
      println!("{:?}", (from, to));
      assert_eq!(self.get(to), val);
    }
  }
  fn _move_inner(&self, from: Pos, to: Pos) {
    println!("_move_inner{:?}", (from, to));
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
            self.apply_move(Move(
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
            self.apply_move(Move(center, rot_axis, from_dir * to_dir * rot_dir));
          }
        } else {
          self._move_inner(from, Pos(2, 2, 1));
          self.apply_moves(INTO_CENTER_REVERSE.clone());
          self.print();
          self._move_inner(Pos(1, 3, 1), to);
        }
      } else {
        if in_center(to) {
          self._move_inner(from, Pos(1, 3, 1));
          self.apply_moves(INTO_CENTER.clone());
          self.print();
          self._move_inner(Pos(2, 2, 1), to);
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
          self.apply_move(Move(center, Axis::Z, f(from) - f(to)));
          if to.2 != from.2 {
            self.apply_move(Move(center, Axis::Y, (to.0 + to.2 - 1) as i8 % 4));
          }
        }
      }
    } else {
      if from == center {
        self.apply_moves(INTO_CENTER_REVERSE.clone());
        self._move_inner(Pos(1, 3, 2), to);
      } else if to == center {
        self._move_inner(from, Pos(1, 3, 2));
        self.apply_moves(INTO_CENTER.clone());
      } else if from.2 != to.2 {
        let mut from = from;
        while from.2 != to.2 {
          let axis = if from.0 == 2 { Axis::Y } else { Axis::X };
          self.apply_move(Move(center, axis, 1));
          from = from.rotate(axis, 1, 5);
        }
        self._move_inner(from, to);
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
        self.apply_move(Move(center, Axis::Z, f(from) - f(to)));
      }
    }
    self.print();
  }
}

impl<T: Cube> _SolveFinal for T {}
impl<T: Cube> SolveFinal for T {}
