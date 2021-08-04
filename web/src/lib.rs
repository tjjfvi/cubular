use cubular_core::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn solve_from_str(str: String) -> Result<String, JsValue> {
  let mut cube = parse_cube(&str).map_err(<_ as Into<JsValue>>::into)?;
  cube.solve();
  Ok(get_moves_str(cube.moves))
}

#[wasm_bindgen]
pub fn make_scrambled_cube() -> String {
  let mut cube = RootCube::solved();
  cube.scramble(1000);
  DisplayCube(&cube).to_string()
}

#[wasm_bindgen]
pub fn apply_moves(cube_str: String, moves_str: String) -> Result<String, JsValue> {
  let mut cube = parse_cube(&cube_str).map_err(<_ as Into<JsValue>>::into)?;
  cube.apply_moves(parse_moves_str(&moves_str)?);
  Ok(DisplayCube(&cube).to_string())
}
