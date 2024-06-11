let inputRequest = false;

/**@type {BoardHook} */
function boardClick(x, y) {
    if (inputRequest) {
        inputRequest = false;
        emit("board-input", {x,y});
    }
}

listen("get-input", (_) => {
    inputRequest = true;
});

listen("create-board", (ev) => {
    initBoard(ev.payload["w"], ev.payload["h"], boardClick);
});

listen("player-move", (e) => {
    let ep = e.payload;
    let x = ep.x;
    let y = ep.y;
    let t = ep.team;
    playBoard.addTo(x, y, t);
    if (playBoard.checkWinner() != -2) {
        emit("move-complete", "true");
    } else {
        emit("move-complete", "false");
    }
})