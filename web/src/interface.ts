
import { inputSpan, focusInputSpan, writeLine, logsDiv } from "./console";
import { cubePre, cube, resetCubePre, paint, getCubeBuffer } from "./cube";

export default () => {
  const charsets = {
    alpha: "0a1b2c3d4e5f6g7h8i",
    zero_mod_nine: "012345678012345678",
    one_mod_nine: "123456789123456789",
  };

  const demoScrambleDelay = 2;
  const demoSolveDelay = 5;

  let charset = "alpha";
  let moveDelay = 5;

  let timeout: number;
  let lastTick = Date.now();
  let deltaTime = 0;

  let demoPhase: null | "scramble" | "solve" = null;

  const history = [""];
  let historyPos = 0;

  inputSpan.textContent = "demo";
  focusInputSpan();
  writeLine(getHelpText());

  tick();

  inputSpan.addEventListener("keydown", e => {
    if (e.key === "Enter") {
      e.preventDefault();
      processCommand(inputSpan.textContent!);
      inputSpan.textContent = "";
    } else if (e.key === "ArrowUp" || e.key === "ArrowDown") {
      e.preventDefault();
      if (historyPos === history.length - 1)
        history[historyPos] = inputSpan.textContent!;
      historyPos += e.key === "ArrowDown" ? 1 : -1;
      if (historyPos < 0) historyPos = 0;
      if (historyPos > history.length - 1) historyPos = history.length - 1;
      inputSpan.textContent = history[historyPos];
      focusInputSpan();
    }
  })

  cubePre.addEventListener("paste" as any, (e: ClipboardEvent) => {
    e.preventDefault();
    let text = e.clipboardData!.getData("text/plain");
    document.execCommand("insertHTML", false, text);

    if (cubePre.textContent === text)
      focusInputSpan();
  });

  cubePre.addEventListener("blur", () => {
    cubePre.attributes.getNamedItem("contenteditable")!.value = "false";
    try {
      cube.set(cubePre.textContent ?? "")
    } catch (e) {
      writeLine(e);
    }
    resetCubePre();
    tick();
  })

  function processCommand(str: string) {
    writeLine("\n> " + str);
    str = str.trim().toLowerCase();
    if (!str) return;
    history[history.length - 1] = str;
    history.push("");
    historyPos = history.length - 1;
    let match;
    const [cmd, ...args] = str.split(" ");
    if (cmd === "?" || cmd === "h" || cmd === "help")
      writeLine(getHelpText());
    else if (cmd === "clear")
      logsDiv.innerHTML = "";
    else if (cmd === "solve")
      cube.solve();
    else if (cmd === "scramble")
      cube.scramble(+args[0] || 1000);
    else if (cmd === "reset")
      cube.reset();
    else if (cmd === "skip")
      cube.flush_all_moves();
    else if (cmd === "stop")
      cube.cancel_queued_moves();
    else if (cmd === "demo") {
      demoPhase = "scramble";
      cube.scramble(1000);
      writeLine("Scrambling with 1000 random moves.");
    }
    else if (cmd === "link") {
      let str = "";
      let cubeBuffer = getCubeBuffer();
      for (const x of cubeBuffer)
        str += x % 9;
      window.location.hash = `#${str}`;
    }
    else if (cmd === "edit") {
      clearTimeout(timeout);
      cubePre.setAttribute("contenteditable", "true");
      cubePre.textContent = cubePre.textContent;
      cubePre.focus();
    }
    else if (/\d\d\d[xyz]\d/.test(cmd))
      try {
        cube.apply_moves(cmd.toUpperCase())
      } catch (e) {
        writeLine(e);
      }
    else if (match = /^(\w+)\s*=\s*(.+)$/.exec(str))
      setConfig(match[1], match[2])
    else if (cmd)
      writeLine(`Unknown command "${str}".\nType "help" for a list of available commands.`)
  }

  function setConfig(key: string, value: string) {
    switch (key) {
      case "charset":
        if (!Object.keys(charsets).includes(value))
          return writeLine(`Invalid value "${value}" for configuration key "charset".`)
        charset = value
        break;
      case "delay":
        moveDelay = +value || 0;
        clearTimeout(timeout);
        tick();
        break;
      default:
        writeLine(`Unknown configuration key "${key}".`)
    }
  }

  function tick() {
    deltaTime = Date.now() - lastTick;
    lastTick = Date.now();
    if (demoPhase === "scramble") {
      let done = !cube.flush_moves(deltaTime / demoScrambleDelay);
      paint(paintCb);
      if (done) {
        // Release main thread so the browser can paint
        setTimeout(() => {
          demoPhase = "solve";
          writeLine("Scrambled.");
          let moveCount = cube.solve();
          writeLine(`Found ${moveCount} move solution.`);
          timeout = setTimeout(() => {
            writeLine("Displaying solution...");
            lastTick = Date.now() - demoSolveDelay;
            tick()
          }, 2000);
        })
      }
      else
        timeout = setTimeout(tick, demoScrambleDelay);
    } else if (demoPhase === "solve") {
      let done = !cube.flush_moves(deltaTime / demoSolveDelay);
      paint(paintCb);
      if (done) {
        demoPhase = null;
        writeLine("Done.");
        timeout = setTimeout(tick, moveDelay);
      } else
        timeout = setTimeout(tick, demoSolveDelay);
    } else {
      if (moveDelay)
        cube.flush_moves(deltaTime / moveDelay);
      else
        cube.flush_all_moves()
      paint(paintCb);
      timeout = setTimeout(tick, moveDelay);
    }
  }
  function getHelpText() {
    return `
cubular v1.0.0

Available commands:
  help               Print this message.
  demo               Scramble and solve the cube.
  edit               Edit the cube configuration.
  scramble [count]   Scramble the puzzle with [count=1000] random moves.
  solve              Solve the puzzle.
  reset              Reset the cube.
  123X1              Apply a move to the puzzle.
  skip               Immediately finish all moves.
  stop               Cancel all queued moves.
  clear              Clear the console.
  [key] = [value]    Change a configuration value.

Configuration:
  charset: ${charset} (${Object.keys(charsets).join(" | ")})
    What characters to use to display the cube. Defaults to "alpha".
    "one_mod_nine" is what's used in the challenge description.
  delay: ${moveDelay} (number)
    The time in milliseconds to wait between each move. Defaults to 5.
`.trimEnd()
  }

  function paintCb(cell: HTMLSpanElement, value: number, solvedValue: number) {
    cell.innerText = (charsets as Record<string, string>)[charset][value];
    cell.className = `
    c${charset === "alpha" ? Math.floor(value / 2) : value % 9}
    ${value === solvedValue ? "solved" : "unsolved"}
  `;
  }
}