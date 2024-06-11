document.getElementById("mopt-done-button").addEventListener("click", () => {hideModal(optionsMenu);});

const globOptions = Object.seal({
    compactMode:false,
});

/**@type {HTMLInputElement} */
const optsCompactMode = document.getElementById("mopts-compact-board");
optsCompactMode.addEventListener("input", () => {
    globOptions.compactMode = optsCompactMode.checked;
    if (gameBoard.classList.contains("M--compact") !== globOptions.compactMode) {
        gameBoard.classList.toggle("M--compact");
    }
});