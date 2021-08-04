use std::collections::HashSet;

use super::*;

pub trait SolveStep {
  fn get_swap<C: Cube>(&self, cube: &C, pos: Pos) -> Option<Swap>;
  fn move_pool<C: Cube>(&self, cube: &mut C, from: Pos, to: Pos);
  fn in_bounds(&self, _pos: Pos) -> bool {
    true
  }
  fn apply_move<C: Cube>(&self, cube: &mut C, m: Move) {
    cube.apply_move(m);
  }
  fn apply<C: Cube>(&self, cube: &mut C) {
    let mut solved = <HashSet<Pos>>::new();
    let mut todo: Vec<_> = cube
      .iter()
      .map(|x| x.0)
      .filter_map(|x| {
        if self.in_bounds(x) {
          self.get_swap(cube, x).and_then(|swap| Some((x, swap)))
        } else {
          None
        }
      })
      .collect();
    todo.sort_by_key(|x| x.1.index);
    for (pos, swap) in todo {
      solved.insert(pos);
      let solved_value = cube.get_solved(pos);
      if cube.get(pos) == solved_value {
        continue;
      }
      let from = cube
        .iter()
        .find(|(p, v)| *v == solved_value && self.in_bounds(*p) && !solved.contains(p))
        .unwrap()
        .0;
      if let Some(from_swap) = self.get_swap(cube, from) {
        for m in from_swap.moves.reverse_moves() {
          self.apply_move(cube, m);
        }
        debug_assert_eq!(cube.get(from_swap.source), solved_value);
        self.move_pool(cube, from_swap.source, swap.source);
      } else {
        self.move_pool(cube, from, swap.source);
      }
      debug_assert_eq!(cube.get(swap.source), solved_value);
      for m in swap.moves {
        self.apply_move(cube, m);
      }
      debug_assert_eq!(cube.get(pos), solved_value);
    }
  }
}

pub trait ApplySolveStep: Cube + Sized {
  fn apply_solve_step<S: SolveStep>(&mut self, solve_step: S) {
    solve_step.apply(self);
  }
}

impl<T: Cube> ApplySolveStep for T {}
