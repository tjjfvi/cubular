
import { consoleDiv, inputSpan, writeLine } from "./console";
import { caption, cube, getCubeBuffer, paint, title } from "./cube";

export default () => {
  const match = /^#(?:([a-zA-Z ]+):)?([0-8]{729})((?:-[1-7]{3}[XYZ]\d(?:@[0-8]{3})*)*-(?:@[0-8]{3})*)?$/.exec(location.hash);
  if (!match) return false

  const [, name, pattern, movesStr = ""] = match;
  const moves = movesStr.split("-").slice(1, -1).map(x => x.slice(0, 5));
  const cubeBuffer = getCubeBuffer();
  const origState = Array(729);
  let underlineCells = movesStr.split("-").slice(1).map(x => x.split("@").slice(1).map(x => parseInt(x, 9))).map(x => Array.from({ length: 729 }, (_, i) => x.includes(i)));
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

  let endMessage = writeLine("\nDone. Highlighting changed pieces.");
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
        cube.apply_moves(moves[moveIndex])
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
        cube.unapply_moves(moves[moveIndex])
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
    cell.innerText = "0a1b2c3d4f5g6h7j8i"[value];
    cell.className = `
      c${value / 2 | 0}
      ${(moves.length && underlineCells[moveIndex + (movePhase === 1 ? 1 : 0)] || [])[index] ? "underline" : ""
      }
  ${moves.length
        ? affectedCells[index]
          ? moveIndex === moves.length
            ? value === origState[index]
              ? "fade"
              : ""
            : movePhase !== -1 && !inMove(moves[moveIndex], index)
              ? "fade"
              : ""
          : "hide"
        : (underlineCells[0] || { [index]: true })[index]
          ? ""
          : "fade"
      }
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
