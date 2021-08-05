
import('./pkg/index.js').then(rs => {
  const cube = rs.ExternCube.new();
  const cubePre = document.getElementById("cube").children[0];
  const consoleDiv = document.getElementById("console");
  const inputSpan = document.querySelector("#input span");
  const logsDiv = document.querySelector("#logs");

  let charset = rs.ExternCharset.Alpha;
  let moveDelay = 10;
  let timeout;

  writeLine(getHelpText());

  updateCubePre();

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
      cube.set(cubePre.innerHTML.replace(/<br>/g, "\n"))
      updateCubePre();
    } catch (e) {
      writeLine(e);
    }
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
      updateCubePre();
    }
    else if (cmd === "scramble") {
      cube.scramble(+args[0] || 100);
      updateCubePre();
    }
    else if (cmd === "reset") {
      cube.reset_state();
      updateCubePre();
    }
    else if (/\d\d\d[xyz]\d/.test(cmd)) {
      try {
        cube.apply_moves(cmd.toUpperCase())
        updateCubePre();
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
        switch (value) {
          case "alpha":
            charset = rs.ExternCharset.Alpha;
            break;
          case "one_mod_nine":
            charset = rs.ExternCharset.OneModNine;
            break;
          case "zero_mod_nine":
            charset = rs.ExternCharset.ZeroModNine;
            break;
          default:
            writeLine(`Invalid value "${value}" for configuration key "charset".`)
        }
        updateCubePre();
        break;
      case "move_delay":
        moveDelay = +value || 0;
        updateCubePre();
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

  function updateCubePre() {
    clearTimeout(timeout)
    if (moveDelay) {
      if (cube.flush_move()) {
        timeout = setTimeout(updateCubePre, moveDelay)
      }
    } else {
      cube.flush_all_moves();
    }
    cubePre.innerText = cube.to_string(charset);
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
  set [key] [value]  Change a configuration value.

Configuration:
  charset: ${(
        charset === rs.ExternCharset.Alpha
          ? "alpha"
          : charset === rs.ExternCharset.OneModNine
            ? "one_mod_nine"
            : "zero_mod_nine"
      )} (alpha | zero_mod_nine | one_mod_nine)
    What characters to use to display the cube. Defaults to "alpha".
    "one_mod_nine" is what's used in the challenge description.
  move_delay: ${moveDelay} (number)
    The time in milliseconds to wait between each move. Defaults to 10.
`.trimEnd()
  }
})
