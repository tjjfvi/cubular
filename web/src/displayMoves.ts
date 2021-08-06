
import { inputSpan, writeLine } from "./console";
import { cube, getCubeBuffer, paint } from "./cube";

export default () => {
  const match = /^#([0-8]{729})((?:-[1-7]{3}[XYZ]\d(?:@[0-8]{3})*)+-(?:@[0-8]{3})*)?$/.exec(location.hash);
  if (!match) return false

  const [, pattern, movesStr = ""] = match;
  const moves = movesStr.split("-").slice(1, -1).map(x => x.slice(0, 5));
  const cubeBuffer = getCubeBuffer();
  const origState = Array(729);
  let underlineCells = movesStr.split("-").slice(1).map(x => x.split("@").slice(1).map(x => parseInt(x, 9))).map(x => Array.from({ length: 729 }, (_, i) => x.includes(i)));
  console.log(underlineCells)
  let affectedCells = Array.from({ length: 729 }, (_, i) => moves.some(m => inMove(m, i)));

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

  cube.cancel_queued_moves();

  if (!moves.length) {
    return false
  }

  Object.assign(document.getElementById("input")!.style, {
    position: "absolute",
    left: "-10000vw",
  })

  writeLine("\nMoves: (use up/down arrows to step through)")
  const moveSpans = moves.map(m => {
    const line = writeLine(" - ")
    const span = document.createElement("span");
    span.textContent = m;
    line.appendChild(span);
    return span;
  });

  let endMessage = writeLine("\nDone. Highlighting changed pieces.");
  endMessage.style.display = "none";

  let moveIndex = 0;

  let showSolved = false;

  // 0: show everything
  // 1: highlight parts that will be affected
  // 2: show result after move
  let movePhase = 0;

  inputSpan.addEventListener("keydown", e => {
    if (e.key === "ArrowDown") {
      movePhase++;
      if (movePhase > 1 && moveIndex === moves.length)
        movePhase--
      if (movePhase === 2) {
        cube.apply_moves(moves[moveIndex])
        moveSpans[moveIndex].style.textDecoration = "line-through"
      }
      if (movePhase === 3) {
        movePhase = 0;
        moveIndex++;
        if (moveIndex === moves.length)
          endMessage.style.display = "inline";
      }
    } else if (e.key === "ArrowUp") {
      movePhase--;
      endMessage.style.display = "none";
      if (movePhase < 0 && moveIndex === 0)
        movePhase++
      if (movePhase === 1) {
        cube.unapply_moves(moves[moveIndex])
        moveSpans[moveIndex].style.textDecoration = "none"
      }
      if (movePhase === -1) {
        movePhase = 2;
        moveIndex--;
      }
    }
    cube.flush_all_moves();
    paint(paintCb);
  })

  paint(paintCb);

  function paintCb(cell: HTMLSpanElement, value: number, solvedValue: number, index: number) {
    cell.innerText = "0a1b2c3d4f5g6h7j8i"[value];
    cell.className = `
      c${value / 2 | 0}
      ${underlineCells[moveIndex + (movePhase === 2 ? 1 : 0)][index] ? "underline" : ""}
      ${affectedCells[index]
        ? moveIndex === moves.length
          ? value === origState[index]
            ? "fade"
            : ""
          : movePhase !== 0 && !inMove(moves[moveIndex], index)
            ? "fade"
            : ""
        : "hide"}
    `;
  }

  function inMove(move: string, index: number) {
    let x = index / 81 | 0;
    let y = (index / 9 | 0) % 9;
    let z = index % 9;
    return (Math.abs(x - +move[0]) <= 1) && (Math.abs(y - +move[1]) <= 1) && (Math.abs(z - +move[2]) <= 1)
  }

  return true
};
