/**@type {HTMLDivElement} */
const gameArea = document.getElementById("game-area");
/**@type {HTMLDivElement} */
const gameBoard = gameArea.children[0];

/**
 * replaces the current board with a new one of the given dimensions
 * @param {number} width
 * @param {number} height
 * @param {BoardHook?} hook
 * @returns {void}
 */
function initBoard(width, height, hook) {
    if (width < 1 || height < 1) {
        throw new RangeError("width and height must be > 0");
    }
    if (width === height && width === 1) {
        throw new Error("1 x 1 boards are not valid");
    }
    if (width > MAX_SIZE || height > MAX_SIZE) {
        throw new Error("width and height cannot be greater than maximum size");
    }
    if (!hook && DEBUG_MODE) {
        hook = (x,y)=>{console.log(`board click: ${x}, ${y}`);};
    }
    // clear the game area
    gameBoard.replaceChildren();
    gameArea.style.cssText = `--cw:${width};--ch:${height};`;
    for (let y = 0; y < height; y ++) {
        for (let x = 0; x < height; x ++) {
            let c = document.createElement("div");
            c.classList.add("board-space");
            c.style.cssText = `--x:${x};--y:${y};--color:${normal_colors["-1"]};`;
            if (hook) {
                c.addEventListener("click", ()=>{hook(x,y);});
            }
            let ct = document.createElement("span");
            ct.textContent = csetraw[0];
            ct.classList.add("board-space-text");
            c.appendChild(ct);
            gameBoard.appendChild(c);
        }
    }
}