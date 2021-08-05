use cubular_core::*;
use std::{collections::VecDeque, intrinsics::transmute, time::Instant};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ExternCube {
  current_state: [[[Value; 9]; 9]; 9],
  queued_state: [[[Value; 9]; 9]; 9],
  queued_moves: VecDeque<Move>,
}

#[wasm_bindgen]
impl ExternCube {
  pub fn new() -> ExternCube {
    ExternCube {
      current_state: *SOLVED,
      queued_state: *SOLVED,
      queued_moves: VecDeque::new(),
    }
  }

  pub fn reset(&mut self) {
    self.current_state = *SOLVED;
    self.queued_state = *SOLVED;
    self.queued_moves.clear();
  }

  pub fn get_state(&self) -> Box<[u8]> {
    Box::new(unsafe { transmute::<_, [u8; 729]>(self.current_state) })
  }

  pub fn set(&mut self, str: &str) -> Result<(), JsValue> {
    self.current_state = parse_cube(str)?;
    self.queued_state = self.current_state;
    self.queued_moves.clear();
    Ok(())
  }

  pub fn flush_moves(&mut self, count: u32) -> bool {
    for _ in 0..count {
      if let Some(m) = self.queued_moves.pop_front() {
        self.current_state.apply_move(m);
      } else {
        return false;
      }
    }
    true
  }

  pub fn flush_all_moves(&mut self) {
    self.current_state = self.queued_state;
    self.queued_moves.clear();
  }

  pub fn solve(&mut self) -> usize {
    <_ as Solve>::solve(self);
    self.queued_moves.len()
  }

  pub fn scramble(&mut self, iterations: u32) {
    <_ as Scramble>::scramble(self, iterations);
  }

  pub fn apply_moves(&mut self, moves_str: String) -> Result<(), JsValue> {
    <_ as Cube>::apply_moves(self, parse_moves_str(&moves_str)?);
    Ok(())
  }
}

impl Cube for ExternCube {
  fn get(&self, pos: Pos) -> Value {
    self.queued_state.get(pos)
  }
  fn get_solved(&self, pos: Pos) -> Value {
    self.queued_state.get_solved(pos)
  }
  fn apply_move(&mut self, m: Move) {
    self.queued_state.apply_move(m.clone());
    self.queued_moves.push_back(m);
  }
  fn size(&self) -> Pos {
    self.queued_state.size()
  }
}
