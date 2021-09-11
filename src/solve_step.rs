use std::collections::HashSet;

use crate::*;

pub trait SolveStep {
  fn classify<C: Cube>(&self, cube: &C, pos: Pos) -> PosClass;
  fn move_pool<C: Cube>(&self, cube: &mut C, from: Pos, to: Pos);
  fn apply_move<C: Cube>(&self, cube: &mut C, m: Move) {
    cube.apply_move(m);
  }
  fn apply<C: Cube>(&self, cube: &mut C) {
    let mut solved = <HashSet<Pos>>::new();
    let mut todo: Vec<_> = cube
      .iter()
      .filter_map(|(pos, _)| match self.classify(cube, pos) {
        PosClass::Active {
          index,
          source,
          moves,
        } => Some((pos, index, source, moves)),
        _ => None,
      })
      .collect();
    todo.sort_by_key(|x| x.1);
    for (pos, _, source, moves) in todo {
      solved.insert(pos);
      let solved_value = cube.get_solved(pos);
      if cube.get(pos) == solved_value {
        continue;
      }
      let from = cube
        .iter()
        .find(|&(p, v)| {
          v == solved_value
            && !matches!(self.classify(cube, p), PosClass::Other)
            && !solved.contains(&p)
        })
        .unwrap()
        .0;
      if let PosClass::Active {
        moves: other_moves,
        source: other_source,
        ..
      } = self.classify(cube, from)
      {
        for m in other_moves.reverse_moves() {
          self.apply_move(cube, m);
        }
        debug_assert_eq!(cube.get(other_source), solved_value);
        self.move_pool(cube, other_source, source);
      } else {
        self.move_pool(cube, from, source);
      }
      debug_assert_eq!(cube.get(source), solved_value);
      for m in moves {
        self.apply_move(cube, m);
      }
      debug_assert_eq!(cube.get(pos), solved_value);
    }
  }
}

pub trait ApplySolveStep: Cube + Sized {
  fn apply_solve_step<S: SolveStep>(&mut self, solve_step: S) -> &mut Self {
    solve_step.apply(self);
    self
  }
}

impl<T: Cube> ApplySolveStep for T {}

#[derive(Debug)]
pub enum PosClass {
  Active {
    index: usize,
    source: Pos,
    moves: Vec<Move>,
  },
  Pool,
  Other,
}

impl PosClass {
  pub fn swap_axes(&mut self, from: Axis, to: Axis) {
    match self {
      Self::Active { source, moves, .. } => {
        *source = source.swap_axes(from, to);
        for m in moves.iter_mut() {
          m.2 = -m.2;
          m.0 = m.0.swap_axes(from, to);
          if m.1 == from {
            m.1 = to;
          } else if m.1 == to {
            m.1 = from;
          } else {
            m.1;
          }
        }
      }
      _ => {}
    }
  }
  pub fn rotate(&mut self, axis: Axis, amount: i8, max: usize) {
    let amount = amount.rem_euclid(4);
    if amount == 0 {
      return;
    }
    match self {
      Self::Active { source, moves, .. } => {
        *source = source.rotate(axis, amount, max);
        for m in moves.iter_mut() {
          m.0 = m.0.rotate(axis, amount, max);
          if match (axis, amount) {
            (Axis::X, 1) | (Axis::Y, 3) => m.1 == Axis::Z,
            (Axis::Y, 1) | (Axis::Z, 3) => m.1 == Axis::X,
            (Axis::Z, 1) | (Axis::X, 3) => m.1 == Axis::Y,
            (_, _) => m.1 != axis,
          } {
            m.2 = -m.2;
          }
          if amount != 2 {
            m.1 = match (axis, m.1) {
              (Axis::Y, Axis::Z) | (Axis::Z, Axis::Y) => Axis::X,
              (Axis::Z, Axis::X) | (Axis::X, Axis::Z) => Axis::Y,
              (Axis::X, Axis::Y) | (Axis::Y, Axis::X) => Axis::Z,
              _ => axis,
            };
          }
        }
      }
      _ => {}
    }
  }
}
