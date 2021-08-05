
import('./pkg/index.js').then(rs => {
  const cube = rs.ExternCube.new();
  const cubePre = document.getElementById("cube").children[0];
  const consoleDiv = document.getElementById("console");
  const inputSpan = document.querySelector("#input span");
  const logsDiv = document.querySelector("#logs");

  writeLine(`
cubular v1.0.0

Type "help" for a list of available commands.
`);

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
      writeLine(helpText);
    else if (cmd === "clear")
      logsDiv.innerHTML = "";
    else if (cmd === "solve") {
      cube.reset_moves();
      cube.solve();
      updateCubePre();
      writeLine(cube.get_moves());
      cube.reset_moves();
    } else if (cmd === "scramble") {
      cube.scramble(+args[0] || 1000);
      updateCubePre();
      cube.reset_moves();
    } else if (cmd === "reset") {
      cube.reset_state();
      updateCubePre();
    } else if (/\d\d\d[xyz]\d/.test(cmd)) {
      try {
        cube.apply_moves(cmd.toUpperCase())
        updateCubePre();
      } catch (e) {
        writeLine(e);
      }
    } else if (cmd) {
      writeLine(`Unknown command "${str}".\nType "help" for a list of available commands.`)
      console.log(str);
    }
  }

  function writeLine(str) {
    let span = document.createElement("span");
    span.textContent = str;
    logsDiv.appendChild(span);
  }

  function updateCubePre() {
    cubePre.innerText = cube.to_string();
  }
})

const helpText = `
Available commands:
  help               Print this message.
  solve              Solve the puzzle.
  scramble [count]   Scramble the puzzle with [count=1000] random moves.
  123X1              Apply a move to the puzzle.
  clear              Clear the console.
  reset              Reset the cube.
`.trim()
