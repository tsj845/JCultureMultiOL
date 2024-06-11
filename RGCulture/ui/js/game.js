class Cell {
    /**
     * @param {number} x
     * @param {number} y
     */
    constructor(x, y) {
        /**@type {number} */
        this.x = x;
        /**@type {number} */
        this.y = y;
        /**@type {number} */
        this.value = 1;
        /**@type {number} */
        this.team = -1;
    }
}

class Board {
    /**
     * @param {number} width
     * @param {number} height
     */
    constructor(width, height) {
        /**
         * @type {number}
         * @constant
         */
        this.w = width;
        /**
         * @type {number}
         * @constant
         */
        this.h = height;
        /**@type {Cell[][]} */
        this.board = [];
        for (let y = 0; y < height; y ++) {
            let a = [];
            for (let x = 0; x < width; x ++) {
                a.push(new Cell(x, y));
            }
            this.board.push(a);
        }
    }
    /**
     * checks if a cell would topple if added to
     * @private
     * @param {Cell} cell the cell to check
     * @returns {boolean}
     */
    willTopple(cell) {
        return cell.value >= (4-(cell.x===0?1:0)-(cell.x===(this.w-1)?1:0)-(cell.y===0?1:0)-(cell.y===(this.h-1)?1:0));
    }
    /**
     * adds the neighbors of the given cell to the given list
     * @param {Cell} cell
     * @param {Cell[]} list list to put the neighbors in
     * @returns {void}
     */
    getNeighbors(cell, list) {
        if (cell.x > 0) {
            list.push(this.board[cell.y][cell.x-1]);
        }
        if (cell.x < (this.w-1)) {
            list.push(this.board[cell.y][cell.x+1]);
        }
        if (cell.y > 0) {
            list.push(this.board[cell.y-1][cell.x]);
        }
        if (cell.y < (this.h-1)) {
            list.push(this.board[cell.y+1][cell.x]);
        }
    }
    checkWinner() {
        let cteam = this.board[0][0].team;
        if (cteam == -1) {
            return -2;
        }
        for (let y = 0; y < this.h; y ++) {
            for (let x = 0; x < this.w; x ++) {
                if (this.board[y][x].team != cteam) {
                    return -2;
                }
            }
        }
        return cteam;
    }
    topple(cell, team) {
        /**@type {Cell[]} */
        let toTopple = [];
        cell.value = 1;
        this.getNeighbors(cell, toTopple);
        while (toTopple.length > 0) {
            const c = toTopple.pop();
            if (c.team != team) {
                // Teams.teams[team].tscore += c.value;
            }
            c.team = team;
            if (this.willTopple(c)) {
                c.value = 1;
                this.getNeighbors(c, toTopple);
            } else {
                c.value ++;
            }
            if (this.checkWinner() > -2) {
                return;
            }
        }
    }
    addTo(x, y, team) {
        let cell = this.board[y][x];
        cell.team = team;
        if (this.willTopple(cell)) {
            this.topple(cell, team);
        } else {
            cell.value ++;
        }
        this.display();
    }
    display() {
        for (let y = 0; y < this.h; y ++) {
            for (let x = 0; x < this.w; x ++) {
                const cell = this.board[y][x];
                const elem = gameBoard.children[y*this.w+x];
                elem.replaceChildren(produceBoardDisplay(cell.value));
                setBoardDisplayTeam(x, y, cell.team);
            }
        }
    }
}