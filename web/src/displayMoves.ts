
import { consoleDiv, inputSpan, writeLine } from "./console";
import { caption, cube, getCubeBuffer, paint, title } from "./cube";
import links from "./links"

export default () => {
  const hash = location.hash.slice(1);
  const hashData = links[location.hash.slice(1)] || location.hash;
  const match = /^(?:([a-zA-Z ]+):)?([0-8]{729})((?:-[1-7]{3}[XYZ]\dt?(?:@@?[0-8]{3})*)*-(?:@@?[0-8]{3})*)?$/.exec(hashData);
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

  caption.innerText = name || (hash === hashData ? "" : hash)

  cube.cancel_queued_moves();

  if (!moves.length) {
    paint(paintCb);
    consoleDiv.style.display = "none";
    return true
  }

  title.innerText = "Goal";
  moves.map(applyMove);
  cube.flush_all_moves();

  document.getElementById("input")!.style.display = "none";

  writeLine("\nMoves: (use up/down arrows to step through)")
  const goalSpan = printCaretLine("Goal");
  const startSpan = printCaretLine("Start");
  const moveSpans = moves.map(printCaretLine);
  const endSpan = printCaretLine("End");

  goalSpan.textContent = ">";

  let moveIndex = moves.length;

  // 0: highlight parts that will be affected
  // 1: show result after move
  let movePhase = 1;

  document.body.addEventListener("keydown", e => {
    if (e.key === "ArrowDown" || e.key === "ArrowRight") {
      e.preventDefault();
      movePhase++;
      if (movePhase > 1 && moveIndex === moves.length){
        moves.slice().reverse().map(unapplyMove);
        movePhase = -1;
        moveIndex = 0;
        title.innerText = "Start";
      }
      else if (movePhase === 1 && moveIndex === moves.length){
        movePhase = 0;
        moveIndex = moves.length;
        title.innerText = "End";
      }
      else if (movePhase === 1) {
        applyMove(moves[moveIndex]);
        title.style.textDecoration = "line-through"
      }
      else if (movePhase === 2) {
        movePhase = 0;
        moveIndex++;
      }
      if(movePhase === 0){
        title.innerText = moves[moveIndex] || "End/Goal"
        title.style.textDecoration = "none"
      }
    } else if (e.key === "ArrowUp" || e.key === "ArrowLeft") {
      e.preventDefault();
      movePhase--;
      if (movePhase === 0 && moveIndex === moves.length)
        movePhase++
      else if (movePhase === 0) {
        unapplyMove(moves[moveIndex]);
        title.style.textDecoration = "none"
      }
      if (movePhase === -1 && moveIndex) {
        movePhase = 1;
        moveIndex--;
        title.innerText = moves[moveIndex] || ""
        title.style.textDecoration = "line-through"
      }
      if (movePhase === -2) {
        moves.map(applyMove);
        moveIndex = moves.length;
        movePhase = 1;
        title.innerText = "Goal";
      }
    }
    cube.flush_all_moves();
    [goalSpan, startSpan, ...moveSpans, endSpan].forEach(x => x.textContent = " ");
    (movePhase === -1 ? startSpan : moveSpans[moveIndex] ?? [endSpan, goalSpan][movePhase]).textContent = movePhase === 0 && moveIndex !== moves.length ? ">" : "="
    paint(paintCb);
  })

  paint(paintCb);

  function paintCb(cell: HTMLSpanElement, value: number, solvedValue: number, index: number) {
    let curMarkCells = (moves.length && markCells[moveIndex + (movePhase === 1 && moveIndex !== moves.length ? 1 : 0)] || []);
    cell.innerText = "0a1b2c3d4e5f6g7h8i"[value];
    cell.className = `
      c${value / 2 | 0}
      ${curMarkCells[index] ? curMarkCells[index] === "@@" ? "mark2" : "mark" : ""}
      ${moves.length
        ? affectedCells[index]
          ? moveIndex === moves.length || movePhase < 0
            ? curMarkCells[index]
              ? ""
              : "fade"
            : !inMove(moves[moveIndex], index)
              ? "fade"
              : ""
          : "hide"
        : (markCells[0] || { [index]: true })[index]
          ? ""
          : "dim"
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

  function applyMove(move: string){
    cube[move.endsWith("t") ? "apply_thin_moves" : "apply_moves"](move.slice(0, 5))
  }

  function unapplyMove(move: string){
    cube[move.endsWith("t") ? "unapply_thin_moves" : "unapply_moves"](move.slice(0, 5))
  }

  function printCaretLine(text: string){
    const line = writeLine(` `)
    const span = document.createElement("span");
    span.textContent = " ";
    line.append(span, " " + text);
    return span;
  }

  return true
};
