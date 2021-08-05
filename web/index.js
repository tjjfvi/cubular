
import('./pkg/index.js').then(rs => {
  const cube = rs.ExternCube.new();
  const cubePre = document.getElementById("cube").children[0];
  const consoleDiv = document.getElementById("console");
  const inputSpan = document.querySelector("#input span");
  const logsDiv = document.querySelector("#logs");

  let cubeCells;
  resetCubePre();

  const charsets = {
    alpha: "0a1b2c3d4e5f6g7h8i",
    zero_mod_nine: "012345678012345678",
    one_mod_nine: "123456789123456789",
  };

  const demoScrambleDelay = 2;
  const demoSolveDelay = 5;

  let charset = "alpha";
  let moveDelay = 5;

  let timeout;
  let lastTick = null;
  let deltaTime = 0;

  let demoPhase = null;

  let history = [""];
  let historyPos = 0;

  inputSpan.textContent = "demo";
  focusInputSpan();
  writeLine(getHelpText());
  tick();

  cubePre.addEventListener("paste", e => {
    e.preventDefault();
    let text = e.clipboardData.getData("text/plain");
    document.execCommand("insertHTML", false, text);

    if (cubePre.innerText === text)
      focusInputSpan();
  });

  inputSpan.addEventListener("paste", e => {
    e.preventDefault();
    let text = e.clipboardData.getData("text/plain");
    document.execCommand("insertHTML", false, text);
  });

  consoleDiv.addEventListener("click", e => {
    if (window.getSelection().type === "Caret") // Allow selection of console text
      focusInputSpan();
  });

  inputSpan.addEventListener("keydown", e => {
    if (e.key === "Enter") {
      e.preventDefault();
      processCommand(inputSpan.textContent);
      inputSpan.textContent = "";
    } else if (e.key === "ArrowUp" || e.key === "ArrowDown") {
      e.preventDefault();
      if (historyPos === history.length - 1)
        history[historyPos] = inputSpan.textContent;
      historyPos += e.key === "ArrowDown" ? 1 : -1;
      if (historyPos < 0) historyPos = 0;
      if (historyPos > history.length - 1) historyPos = history.length - 1;
      inputSpan.textContent = history[historyPos];
      focusInputSpan();
    }
  })

  cubePre.addEventListener("blur", () => {
    cubePre.attributes.contenteditable.value = false;
    try {
      cube.set(cubePre.innerText)
    } catch (e) {
      writeLine(e);
    }
    resetCubePre();
    tick();
  })

  function processCommand(str) {
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
    else if (cmd === "edit") {
      clearTimeout(timeout);
      cubePre.attributes.contenteditable.value = true;
      cubePre.innerText = cubePre.innerText;
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

  function setConfig(key, value) {
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

  function writeLine(str) {
    let span = document.createElement("span");
    span.textContent = str;
    logsDiv.appendChild(span);
  }

  function tick() {
    deltaTime = Date.now() - lastTick;
    lastTick = Date.now();
    if (demoPhase === "scramble") {
      let done = !cube.flush_moves(deltaTime / demoScrambleDelay);
      paint();
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
      paint();
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
      paint();
      timeout = setTimeout(tick, moveDelay);
    }
  }

  function paint() {
    let data = cube.get_state();
    cubeCells.forEach((grid, x) => {
      grid.forEach((row, y) => {
        row.forEach((cell, z) => {
          let value = data[x * 81 + y * 9 + z];
          let solvedValue = (x + y + z) % 18;
          cell.innerText = charsets[charset][value];
          cell.style.color = [
            "#d63031",
            "#e67e22",
            "#f39c12",
            "#f1c40f",
            "#2ecc71",
            "#27ae60",
            "#3498db",
            "#2980b9",
            "#8e44ad",
          ][charset === "alpha" ? Math.floor(value / 2) : value % 9]
          cell.style.opacity = value === solvedValue ? "1" : ".6";
        })
      })
    })
  }

  function resetCubePre() {
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
    function span(str) {
      let el = document.createElement("span");
      el.innerText = str;
      return el;
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

  function focusInputSpan() {
    inputSpan.focus();
    if (inputSpan.childNodes.length) {
      let selection = window.getSelection()
      let range = document.createRange();
      range.setStart(inputSpan.childNodes[0], inputSpan.innerText.length)
      range.collapse(true)
      selection.removeAllRanges()
      selection.addRange(range)
    }
  }
})
