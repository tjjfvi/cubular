
import { consoleDiv, inputSpan, writeLine } from "./console";
import { caption, cube, getCubeBuffer, paint, title } from "./cube";

export default () => {
  const match = /^#(?:([a-zA-Z ]+):)?([0-8]{729})((?:-[1-7]{3}[XYZ]\dt?(?:@@?[0-8]{3})*)*-(?:@@?[0-8]{3})*)?$/.exec(location.hash);
  if (!match) return false

  const [, name, pattern, movesStr = ""] = match;
  const moves = movesStr.split("-").slice(1, -1).map(x => x.split("@")[0]);
  const cubeBuffer = getCubeBuffer();
  const origState = Array(729);
  let markCells = movesStr
    .split("-")
    .slice(1)
    .map(x => (x.match(/@@?\d\d\d/g) ?? []).map(x => [x.slice(0, -3), parseInt(x.slice(-3), 9)] as const))
    .map(x => Array.from({ length: 729 }, (_, i) => x.find(x => x[1] === i)?.[0]));
  let affectedCells = Array.from({ length: 729 }, (_, i) => !moves.length || moves.some(m => inMove(m, i)));

  for (let x = 0; x < 9; x++) {
    for (let y = 0; y < 9; y++) {
      for (let z = 0; z < 9; z++) {
        const i = x * 81 + y * 9 + z;
        let value = +pattern[i];
        if (value % 2 !== (x + y + z) % 2) value += 9
        origState[i] = cubeBuffer[i] = value
      }
    }
  }

  title.innerText = name || ""

  cube.cancel_queued_moves();

  document.getElementById("input")!.style.display = "none";

  writeLine("\nMoves: (use up/down arrows to step through)")
  const moveSpans = moves.map((m, i) => {
    const line = writeLine(` ${(i + 1).toString().padStart(moves.length.toString().length)}. `)
    const span = document.createElement("span");
    span.textContent = m;
    line.appendChild(span);
    return span;
  });

  let endMessage = writeLine("\nDone.");
  endMessage.style.display = "none";

  let moveIndex = 0;

  // 0: highlight parts that will be affected
  // 1: show result after move
  let movePhase = -1;

  if (!moves.length)
    consoleDiv.style.display = "none";

  caption.innerText = moves[moveIndex] || ""

  document.body.addEventListener("keydown", e => {
    if (e.key === "ArrowDown") {
      e.preventDefault();
      movePhase++;
      if (movePhase > 1 && moveIndex === moves.length)
        movePhase--
      if (movePhase === 1) {
        cube[moves[moveIndex].endsWith("t") ? "apply_thin_moves" : "apply_moves"](moves[moveIndex].slice(0, 5))
        caption.style.textDecoration = moveSpans[moveIndex].style.textDecoration = "line-through"
      }
      if (movePhase === 2) {
        movePhase = 0;
        moveIndex++;
        if (moveIndex === moves.length)
          endMessage.style.display = "inline";
        caption.innerText = moves[moveIndex] || ""
        caption.style.textDecoration = "none"
      }
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      movePhase--;
      endMessage.style.display = "none";
      if (movePhase < -1 && moveIndex === 0)
        movePhase++
      else if (movePhase === 0) {
        cube[moves[moveIndex].endsWith("t") ? "unapply_thin_moves" : "unapply_moves"](moves[moveIndex].slice(0, 5))
        caption.style.textDecoration = moveSpans[moveIndex].style.textDecoration = "none"
      }
      if (movePhase === -1 && moveIndex) {
        movePhase = 1;
        moveIndex--;
        caption.innerText = moves[moveIndex] || ""
        caption.style.textDecoration = "line-through"
      }
    }
    cube.flush_all_moves();
    paint(paintCb);
  })

  paint(paintCb);

  function paintCb(cell: HTMLSpanElement, value: number, solvedValue: number, index: number) {
    let curMarkCells = (moves.length && markCells[moveIndex + (movePhase === 1 ? 1 : 0)] || []);
    cell.innerText = "0a1b2c3d4f5g6h7j8i"[value];
    cell.className = `
      c${value / 2 | 0}
      ${curMarkCells[index] ? curMarkCells[index] === "@@" ? "mark2" : "mark" : ""}
      ${moves.length
        ? affectedCells[index]
          ? moveIndex === moves.length
            ? curMarkCells[index]
              ? ""
              : "fade"
            : movePhase !== -1 && !inMove(moves[moveIndex], index)
              ? "fade"
              : ""
          : "hide"
        : (markCells[0] || { [index]: true })[index]
          ? ""
          : "fade"
      }
  `;
  }

  function inMove(move: string, index: number) {
    let x = index / 81 | 0;
    let y = (index / 9 | 0) % 9;
    let z = index % 9;
    let in3x3x3 = (Math.abs(x - +move[0]) <= 1) && (Math.abs(y - +move[1]) <= 1) && (Math.abs(z - +move[2]) <= 1);
    let in1x3x3 = in3x3x3 && { X: x === +move[0], Y: y === +move[1], Z: z === +move[2] }[move[3]]
    let isThinMove = move.endsWith("t");
    return isThinMove ? in1x3x3 : in3x3x3;
  }

  return true
};
