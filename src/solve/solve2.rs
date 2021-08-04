use super::*;
use lazy_static::lazy_static;

pub trait Solve2: Cube + Sized {
  fn solve2(&self) {
    self.print();
    self._solve_cross();
  }
}

lazy_static! {
  static ref INNER_POSS: Vec<Pos> = (3..6)
    .flat_map(|x| (3..6).flat_map(move |y| (3..6).map(move |z| Pos(x, y, z))))
    .collect();
}

trait _Solve2: Cube + Sized {
  fn _apply_thin_move(&self, Move(center, axis, amount): Move) {
    println!("{:?}", Move(center, axis, amount));
    let mut offcenter = center;
    offcenter[axis] -= 1;
    let offaxis = if axis == Axis::X { Axis::Y } else { Axis::X };
    self.apply_move(Move(offcenter, axis, amount));
    self.apply_move(Move(center, offaxis, 2));
    self.apply_move(Move(offcenter, axis, -amount));
    self.apply_move(Move(center, offaxis, 2));
    self.apply_move(Move(center, axis, -amount));
    self.print();
  }
  fn _apply_thin_moves(&self, moves: Vec<Move>) {
    for m in moves {
      self._apply_thin_move(m);
    }
  }
  fn _solve_cross(&self) {
    for pos in vec![
      Pos(3, 4, 4),
      Pos(5, 4, 4),
      Pos(4, 3, 4),
      Pos(4, 5, 4),
      Pos(4, 4, 3),
      Pos(4, 4, 5),
    ] {
      self._solve_cross_piece(pos)
    }
  }
  fn _solve_cross_piece(&self, pos: Pos) {
    let value = self.get_solved(pos);
    if self.get(pos) == value {
      return;
    }
    let from = *INNER_POSS.iter().find(|x| self.get(**x) == value).unwrap();
    dbg!(from, pos);
    let from_swap = self._get_swap(from);
    let to_swap = self._get_swap(pos);
    self._apply_thin_moves(from_swap.moves.reverse_moves());
    self._move_corner(dbg!(from_swap.source), dbg!(to_swap.source));
    self._apply_thin_moves(to_swap.moves);
  }
  fn _move_corner(&self, from: Pos, to: Pos) {
    fn f(pos: Pos) -> i8 {
      if pos.0 == 3 {
        if pos.1 == 3 {
          0
        } else {
          1
        }
      } else {
        if pos.1 == 3 {
          3
        } else {
          2
        }
      }
    }
    self._apply_thin_move(Move(Pos(4, 4, from.2), Axis::Z, f(from) - f(to)));
    if to.2 != from.2 {
      self._apply_thin_move(Move(Pos(4, to.1, 4), Axis::Y, (to.0 + to.2 - 1) as i8 % 4));
    }
  }
  fn _get_swap(&self, pos: Pos) -> Swap {
    assert_eq!(pos.parity(), 1);
    match pos {
      Pos(x, 4, 4) => Swap {
        index: 0,
        source: Pos(x, 3, 3),
        moves: vec![
          Move(Pos(x, 4, 3), Axis::X, 1),
          Move(Pos(x, 3, 4), Axis::X, -1),
          Move(Pos(x, 4, 3), Axis::X, -1),
          Move(Pos(x, 3, 4), Axis::X, 1),
        ],
      },
      Pos(4, y, 4) => Swap {
        index: 0,
        source: Pos(3, y, 3),
        moves: vec![
          Move(Pos(4, y, 3), Axis::Y, 1),
          Move(Pos(3, y, 4), Axis::Y, -1),
          Move(Pos(4, y, 3), Axis::Y, -1),
          Move(Pos(3, y, 4), Axis::Y, 1),
        ],
      },
      Pos(4, 4, z) => Swap {
        index: 0,
        source: Pos(3, 3, z),
        moves: vec![
          Move(Pos(4, 3, z), Axis::Z, 1),
          Move(Pos(3, 4, z), Axis::Z, -1),
          Move(Pos(4, 3, z), Axis::Z, -1),
          Move(Pos(3, 4, z), Axis::Z, 1),
        ],
      },
      // not in cross
      _ => Swap {
        index: 0,
        source: pos,
        moves: vec![],
      },
    }
  }
}

impl<T: Cube> _Solve2 for T {}
impl<T: Cube> Solve2 for T {}
