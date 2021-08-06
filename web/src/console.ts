
export const consoleDiv = document.getElementById("console")!;
export const inputSpan = document.querySelector<HTMLSpanElement>("#input span")!;
export const logsDiv = document.querySelector<HTMLDivElement>("#logs")!;

inputSpan.addEventListener("paste" as any, (e: ClipboardEvent) => {
  e.preventDefault();
  const text = e.clipboardData!.getData("text/plain");
  document.execCommand("insertHTML", false, text);
});

consoleDiv.addEventListener("click", e => {
  if (window.getSelection()!.type === "Caret") // Allow selection of console text
    focusInputSpan();
});

export function writeLine(str: string) {
  let span = document.createElement("span");
  span.textContent = str;
  logsDiv.appendChild(span);
  return span;
}

export function focusInputSpan() {
  inputSpan.focus();
  if (inputSpan.childNodes.length) {
    const selection = window.getSelection()!;
    const range = document.createRange();
    range.setStart(inputSpan.childNodes[0], inputSpan.innerText.length)
    range.collapse(true)
    selection.removeAllRanges()
    selection.addRange(range)
  }
}