use tap::Pipe;

use super::*;
use std::collections::HashSet;

pub trait SolveStep {
  fn get_solved(&mut self) -> &mut HashSet<Pos>;
  fn in_bounds(&mut self, _pos: Pos) -> bool {
    true
  }
  fn can_mutate(&mut self, pos: Pos) -> bool {
    !self.get_solved().contains(&pos) && self.in_bounds(pos)
  }
  fn get_swap<C: Cube>(&mut self, cube: &C, pos: Pos) -> Option<Swap>;
  fn move_pool<C: Cube>(&mut self, cube: &mut C, from: Pos, to: Pos);
  fn apply_move<C: Cube>(&mut self, cube: &mut C, m: Move) {
    cube.apply_move(m);
  }
  fn find_piece<C: Cube>(&mut self, cube: &mut C, value: N) -> Pos {
    cube
      .iter()
      .find(|(p, v)| *v == value && self.can_mutate(*p))
      .unwrap()
      .0
  }
  fn apply<C: Cube>(&mut self, cube: &mut C) {
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
      self.get_solved().insert(pos);
      let solved_value = cube.get_solved(pos);
      if cube.get(pos) == solved_value {
        continue;
      }
      let from = self.find_piece(cube, solved_value);
      if let Some(from_swap) = self.get_swap(cube, from) {
        for m in from_swap.moves.reverse_moves() {
          self.apply_move(cube, m);
        }
        // assert_eq!(cube.get(from_swap.source), solved_value);
        self.move_pool(cube, from_swap.source, swap.source);
      } else {
        self.move_pool(cube, from, swap.source);
      }
      // assert_eq!(cube.get(swap.source), solved_value);
      for m in swap.moves {
        self.apply_move(cube, m);
      }
      // assert_eq!(cube.get(pos), solved_value);
    }
  }
}

pub trait ApplySolveStep: Cube + Sized {
  fn apply_solve_step<S: SolveStep + Default>(&mut self) {
    S::default().apply(self);
  }
}

impl<T: Cube> ApplySolveStep for T {}
