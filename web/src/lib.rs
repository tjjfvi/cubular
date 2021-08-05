use cubular_core::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ExternCube(RootCube);

#[wasm_bindgen]
pub enum ExternCharset {
  Alpha,
  ZeroModNine,
  OneModNine,
}

#[wasm_bindgen]
impl ExternCube {
  pub fn new() -> ExternCube {
    ExternCube(RootCube::solved())
  }

  pub fn reset_state(&mut self) {
    self.0.values = *SOLVED;
    self.0.moves = vec![];
  }

  pub fn get_moves(&self) -> String {
    get_moves_str(&self.0.moves)
  }

  pub fn reset_moves(&mut self) {
    self.0.reset_moves();
  }

  pub fn set(&mut self, str: &str) -> Result<(), JsValue> {
    parse_cube(&mut self.0, str)?;
    Ok(())
  }

  pub fn solve(&mut self) {
    self.0.solve()
  }

  pub fn scramble(&mut self, iterations: u32) {
    self.0.scramble(iterations)
  }

  pub fn to_string(&self, charset: ExternCharset) -> String {
    DisplayCube(
      &self.0,
      match charset {
        ExternCharset::Alpha => ValueCharset::Alpha,
        ExternCharset::OneModNine => ValueCharset::OneModNine,
        ExternCharset::ZeroModNine => ValueCharset::ZeroModNine,
      },
    )
    .to_string()
  }

  pub fn apply_moves(&mut self, moves_str: String) -> Result<(), JsValue> {
    self.0.apply_moves(parse_moves_str(&moves_str)?);
    Ok(())
  }
}
