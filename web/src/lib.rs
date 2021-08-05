use cubular_core::*;
use std::collections::VecDeque;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ExternCube {
  current_state: [[[Value; 9]; 9]; 9],
  queued_state: [[[Value; 9]; 9]; 9],
  queued_moves: VecDeque<Move>,
}

#[wasm_bindgen]
pub enum ExternCharset {
  Alpha,
  ZeroModNine,
  OneModNine,
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

  pub fn set(&mut self, str: &str) -> Result<(), JsValue> {
    parse_cube(&mut self.current_state, str)?;
    self.queued_state = self.current_state;
    self.queued_moves.clear();
    Ok(())
  }

  pub fn flush_move(&mut self) -> bool {
    if let Some(m) = self.queued_moves.pop_front() {
      self.current_state.apply_move(m);
      true
    } else {
      false
    }
  }

  pub fn flush_all_moves(&mut self) {
    self.current_state = self.queued_state;
    self.queued_moves.clear();
  }

  pub fn solve(&mut self) {
    <_ as Solve>::solve(self);
  }

  pub fn scramble(&mut self, iterations: u32) {
    <_ as Scramble>::scramble(self, iterations);
  }

  pub fn to_string(&self, charset: ExternCharset) -> String {
    DisplayCube(
      &self.current_state,
      match charset {
        ExternCharset::Alpha => ValueCharset::Alpha,
        ExternCharset::OneModNine => ValueCharset::OneModNine,
        ExternCharset::ZeroModNine => ValueCharset::ZeroModNine,
      },
    )
    .to_string()
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
