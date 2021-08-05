
import('./pkg/index.js').then(rs => {
  const cube = rs.ExternCube.new();
  const cubePre = document.getElementById("cube").children[0];
  const consoleDiv = document.getElementById("console");
  const inputSpan = document.querySelector("#input span");
  const logsDiv = document.querySelector("#logs");

  const charsets = {
    alpha: "0a1b2c3d4e5f6g7h8i",
    zero_mod_nine: "012345678012345678",
    one_mod_nine: "123456789123456789",
  };

  let cubeCells;
  let charset = "alpha";
  let colors = "solved";
  let moveDelay = 10;
  let lastTick = null;
  let timeout;

  writeLine(getHelpText());

  resetCubePre();
  updateCubeCells();

  document.querySelector("[contenteditable]").addEventListener("paste", function (e) {
    e.preventDefault();
    var text = "";
    if (e.clipboardData && e.clipboardData.getData) {
      text = e.clipboardData.getData("text/plain");
    } else if (window.clipboardData && window.clipboardData.getData) {
      text = window.clipboardData.getData("Text");
    }

    document.execCommand("insertHTML", false, text);
  });

  consoleDiv.addEventListener("click", e => {
    inputSpan.focus()
  });

  inputSpan.addEventListener("keydown", e => {
    if (e.key === "Enter") {
      e.preventDefault();
      processCommand(inputSpan.textContent);
      inputSpan.textContent = "";
      inputSpan.blur()
      inputSpan.focus()
    }
  })

  cubePre.addEventListener("blur", () => {
    writeLine("\n> [edited cube configuration]");
    try {
      cube.set(cubePre.innerText)
    } catch (e) {
      writeLine(e);
    }
    resetCubePre();
    updateCubeCells();
  })

  function processCommand(str) {
    writeLine("\n> " + str);
    str = str.trim().toLowerCase();
    const [cmd, ...args] = str.split(" ");
    if (cmd === "?" || cmd === "h" || cmd === "help")
      writeLine(getHelpText());
    else if (str === "[edited cube configuration]") {
      writeLine("Lies.")
    }
    else if (cmd === "clear")
      logsDiv.innerHTML = "";
    else if (cmd === "solve") {
      cube.solve();
      updateCubeCells();
    }
    else if (cmd === "scramble") {
      cube.scramble(+args[0] || 100);
      updateCubeCells();
    }
    else if (cmd === "reset") {
      cube.reset();
      updateCubeCells();
    } else if (cmd === "skip") {
      cube.flush_all_moves();
      updateCubeCells();
    }
    else if (/\d\d\d[xyz]\d/.test(cmd)) {
      try {
        cube.apply_moves(cmd.toUpperCase())
        updateCubeCells();
      } catch (e) {
        writeLine(e);
      }
    }
    else if (cmd === "set") {
      setConfig(args[0], args[1])
    }
    else if (cmd) {
      writeLine(`Unknown command "${str}".\nType "help" for a list of available commands.`)
      console.log(str);
    }
  }

  function setConfig(key, value) {
    switch (key) {
      case "charset":
        if (!Object.keys(charsets).includes(value))
          return writeLine(`Invalid value "${value}" for configuration key "charset".`)
        charset = value
        updateCubeCells();
        break;
      case "move_delay":
        moveDelay = +value || 0;
        lastTick = null;
        updateCubeCells();
        break;
      case "colors":
        if (!["none", "solved", "value"].includes(value))
          return writeLine(`Invalid value "${value}" for configuration key "colors".`);
        colors = value;
        updateCubeCells();
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

  function updateCubeCells() {
    clearTimeout(timeout)
    if (moveDelay) {
      const count = lastTick ? Math.round((Date.now() - lastTick) / moveDelay) : 1;
      if (cube.flush_moves(count)) {
        timeout = setTimeout(updateCubeCells, moveDelay)
        lastTick = Date.now();
      } else {
        lastTick = null;
      }
    } else {
      lastTick = null
      cube.flush_all_moves();
    }

    let data = cube.get_state();
    cubeCells.forEach((a, x) => {
      a.forEach((a, y) => {
        a.forEach(
          /** @param cell {HTMLElement} */
          (cell, z) => {
            let value = data[x * 81 + y * 9 + z];
            let solvedValue = (x + y + z) % 18;
            cell.innerText = charsets[charset][value];
            cell.style.color = colors === "solved"
              ? value === solvedValue ? "#27ae60" : "inherit"
              : colors === "value"
                ? [
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
                : "inherit"
          })
      })
    })
  }

  function resetCubePre() {
    cubeCells = Array.from({ length: 9 }, () => Array.from({ length: 9 }, () => Array(9)));
    cubePre.innerHTML = "";
    for (let i of [0, 3, 6]) {
      if (i !== 0) cubePre.appendChild(text("\n\n"))
      for (let y of [0, 1, 2, 3, 4, 5, 6, 7, 8]) {
        if (y !== 0) cubePre.appendChild(text("\n"))
        for (let z of [i, i + 1, i + 2]) {
          if (z !== i) cubePre.appendChild(text("  "))
          for (let x of [0, 1, 2, 3, 4, 5, 6, 7, 8]) {
            if (x !== 0) cubePre.appendChild(text(" "))
            let cell = text(".")
            cubeCells[x][y][z] = cell;
            cubePre.appendChild(cell);
          }
        }
      }
    }
    function text(str) {
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
  solve              Solve the puzzle.
  scramble [count]   Scramble the puzzle with [count=100] random moves.
  123X1              Apply a move to the puzzle.
  clear              Clear the console.
  reset              Reset the cube.
  skip               Immediately finish all moves.
  set [key] [value]  Change a configuration value.

Configuration:
  charset: ${charset} (${Object.keys(charsets).join(" | ")})
    What characters to use to display the cube. Defaults to "alpha".
    "one_mod_nine" is what's used in the challenge description.
  move_delay: ${moveDelay} (number)
    The time in milliseconds to wait between each move. Defaults to 10.
  colors: ${colors} (none | solved | value)
    How to color the pieces. Defaults to "solved".
    "none" gives no coloration.
    "solved" colors the solved pieces green.
    "value" assigns a unique color to each value.
`.trimEnd()
  }
})
