use std::collections::HashSet;

use crate::*;
use lazy_static::lazy_static;

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
  }
  fn _get_swap(&self, pos: Pos) -> Swap {
    match pos {
      p if in_inner(p) => Swap {
        order: 0,
        source: p,
        moves: vec![],
      },
      p if p.0 >= 3 || p.1 >= 3 => {
        println!("Z1");
        self
          ._get_swap(pos.rotate(Axis::Z, 1, 5))
          .rotate(Axis::Z, -1, 5)
      }
      p if p.2 >= 1 => {
        let axis = if p.1 == 0 { Axis::X } else { Axis::Y };
        self._get_swap(pos.rotate(axis, 1, 5)).rotate(axis, -1, 5)
      }
      p if p.1 > p.0 => {
        println!("sXY");
        self
          ._get_swap(pos.swap_axes(Axis::X, Axis::Y))
          .swap_axes(Axis::X, Axis::Y)
      }
      Pos(0, 0, 0) => Swap {
        order: 0,
        source: Pos(2, 2, 2),
        moves: vec![
          Move(Pos(1, 1, 1), Axis::Z, 2),
          Move(Pos(1, 1, 1), Axis::X, 1),
        ],
      },
      Pos(1, 0, 0) => Swap {
        order: 1,
        source: Pos(2, 3, 2),
        moves: vec![
          Move(Pos(1, 1, 1), Axis::Z, 1),
          Move(Pos(1, 1, 1), Axis::Y, 1),
          Move(Pos(2, 2, 2), Axis::Z, 2),
          Move(Pos(1, 1, 1), Axis::Y, -1),
          Move(Pos(1, 1, 1), Axis::Z, -1),
        ],
      },
      Pos(2, 0, 0) => Swap {
        order: 2,
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
      Pos(1, 1, 0) => Swap {
        order: 3,
        source: Pos(3, 3, 2),
        moves: vec![
          Move(Pos(1, 1, 1), Axis::Z, 1),
          Move(Pos(1, 1, 1), Axis::Y, 2),
          Move(Pos(2, 2, 2), Axis::Z, 2),
          Move(Pos(1, 1, 1), Axis::Y, 2),
          Move(Pos(1, 1, 1), Axis::Z, -1),
        ],
      },
      Pos(2, 1, 0) => Swap {
        order: 4,
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
      Pos(2, 2, 0) => Swap {
        order: 5,
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
      _ => todo!("{:?}", pos),
    }
  }
  fn _solve(&self) {
    self.print();
    let mut solved: HashSet<Pos> = HashSet::new();
    let mut todo = self
      .iter()
      .filter(|x| !in_inner(x.0))
      .map(|x| (x.0, self._get_swap(x.0)))
      .collect::<Vec<_>>();
    todo.sort_by_key(|x| x.1.order);
    for (pos, swap) in todo {
      self._solve_piece(pos, swap, &mut solved);
    }
    self.print();
  }
  fn _solve_piece(&self, pos: Pos, to_swap: Swap, solved: &mut HashSet<Pos>) {
    println!("\n\n\n\n\n\n\n\n");
    self.print();
    solved.insert(pos);
    let value = self.get_solved(pos);
    if self.get(pos) == value {
      println!("already solved {:?} {:?}", pos, value);
      return;
    }
    let from = (1..=3)
      .flat_map(|x| (1..=3).flat_map(move |y| (1..=3).map(move |z| Pos(x, y, z))))
      .find(|p| self.get(*p) == value)
      .or_else(|| {
        self
          .iter()
          .find(|(p, n)| *n == value && !solved.contains(p))
          .map(|x| x.0)
      })
      .unwrap();
    let from_swap = self._get_swap(from);
    dbg!(from, &from_swap, self.get(from_swap.source));
    self.apply_moves(from_swap.moves.reverse_moves());
    dbg!(self.get(from_swap.source));
    self._move_inner(from_swap.source, to_swap.source);
    dbg!(to_swap.source, self.get(to_swap.source));
    self.print();
    self.apply_moves(to_swap.moves);
    println!("solved {:?} {:?}", pos, value);
    self.print();
    for p in solved.iter() {
      assert_eq!(self.get(*p), self.get_solved(*p), "{:?}", p);
    }
  }
}

impl<T: Cube> _SolveFinal for T {}
impl<T: Cube> SolveFinal for T {}

fn in_inner(p: Pos) -> bool {
  p.0 >= 1 && p.0 <= 3 && p.1 >= 1 && p.1 <= 3 && p.2 >= 1 && p.2 <= 3
}

#[derive(Debug)]
struct Swap {
  pub order: u8,
  pub source: Pos,
  pub moves: Vec<Move>,
}

impl Swap {
  pub fn swap_axes(mut self, from: Axis, to: Axis) -> Self {
    self.source = self.source.swap_axes(from, to);
    for m in self.moves.iter_mut() {
      m.0 = m.0.swap_axes(from, to);
      if m.1 == from {
        m.1 = to;
      } else if m.1 == to {
        m.1 = from;
      } else {
        m.1;
        m.2 = -m.2;
      }
    }
    self
  }
  pub fn rotate(mut self, axis: Axis, amount: i8, max: usize) -> Self {
    let amount = amount.rem_euclid(4);
    if amount == 0 {
      return self;
    }
    self.source = self.source.rotate(axis, amount, max);
    for m in self.moves.iter_mut() {
      m.0 = m.0.rotate(axis, amount, max);
      if amount != 2 {
        m.1 = match (axis, m.1) {
          (Axis::Y, Axis::Z) | (Axis::Z, Axis::Y) => Axis::X,
          (Axis::X, Axis::Z) | (Axis::Z, Axis::X) => Axis::Y,
          (Axis::X, Axis::Y) | (Axis::Y, Axis::X) => Axis::Z,
          _ => axis,
        };
      }
      if m.1
        == match axis {
          Axis::X => Axis::Y,
          Axis::Y => Axis::Z,
          Axis::Z => Axis::X,
        }
      {
        m.2 = -m.2;
      }
    }
    self
  }
}
