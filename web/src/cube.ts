
import * as rs from "../pkg/index.js";
import * as wasm from "../pkg/index_bg.wasm";

export const cube = rs.ExternCube.new();
export const cubePre = document.getElementById("cube")!.children[0] as HTMLPreElement;
export let cubeCells: HTMLSpanElement[][][];

cube.reset();
resetCubePre();

export function resetCubePre() {
  cubeCells = Array.from({ length: 9 }, () => Array.from({ length: 9 }, () => Array(9)));
  cubePre.innerHTML = "";
  for (let i of [0, 3, 6]) {
    if (i !== 0) cubePre.appendChild(span("\n\n"))
    for (let y of [0, 1, 2, 3, 4, 5, 6, 7, 8]) {
      if (y !== 0) cubePre.appendChild(span("\n"))
      for (let z of [i, i + 1, i + 2]) {
        if (z !== i) cubePre.appendChild(span("  "))
        for (let x of [0, 1, 2, 3, 4, 5, 6, 7, 8]) {
          if (x !== 0) cubePre.appendChild(span(" "))
          let cell = span(".")
          cubeCells[x][y][z] = cell;
          cubePre.appendChild(cell);
        }
      }
    }
  }
  function span(str: string) {
    let el = document.createElement("span");
    el.innerText = str;
    return el;
  }
}

export function paint(cb: (cell: HTMLSpanElement, value: number, solvedValue: number, index: number) => void) {
  const cubeBuffer = getCubeBuffer();
  cubeCells.forEach((grid, x) => {
    grid.forEach((row, y) => {
      row.forEach((cell, z) => {
        let index = x * 81 + y * 9 + z;
        let value = cubeBuffer[index];
        let solvedValue = (x + y + z) % 18;
        cb(cell, value, solvedValue, index);
      })
    })
  })
}

export function getCubeBuffer() {
  return new Uint8Array(wasm.memory.buffer, cube.get_state_ptr(), 729)
}
