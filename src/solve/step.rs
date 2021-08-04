use std::collections::HashSet;

use tap::Pipe;

use super::*;

pub trait SolveStep {
  fn get_solved(&mut self) -> &mut HashSet<Pos>;
  fn get_swap<C: Cube>(&mut self, cube: &C, pos: Pos) -> Option<Swap>;
  fn move_pool<C: Cube>(&mut self, cube: &C, from: Pos, to: Pos);
  fn apply_move<C: Cube>(&mut self, cube: &C, m: Move) {
    cube.apply_move(m);
  }
  fn find_piece<C: Cube>(&mut self, cube: &C, value: N) -> Pos {
    let solved = self.get_solved();
    cube
      .iter()
      .find(|(p, v)| *v == value && !solved.contains(p))
      .unwrap()
      .0
  }
  fn apply<C: Cube>(&mut self, cube: &C) {
    let mut todo: Vec<_> = cube
      .iter()
      .map(|x| x.0)
      .filter_map(|x| self.get_swap(cube, x).and_then(|swap| Some((x, swap))))
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
        self.move_pool(cube, from_swap.source, swap.source);
      } else {
        self.move_pool(cube, from, swap.source);
      }
      for m in swap.moves {
        self.apply_move(cube, m);
      }
    }
  }
}

pub trait ApplySolveStep: Cube + Sized {
  fn apply_solve_step<S: SolveStep + Default>(&self) {
    S::default().apply(self);
  }
}

impl<T: Cube> ApplySolveStep for T {}
