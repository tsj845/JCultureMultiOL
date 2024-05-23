package JCRoot.game;

import JCRoot.Host;

public class Game {
    public final int width, height, players;
    public final Board board;
    public int cplayer = 0;
    public Game(int w, int h, int p) {
        width = w;
        height = h;
        players = p;
        board = new Board(w, h, p);
    }
    public boolean validate(int x, int y, int team) {
        if (x < 0 || y < 0 || x >= width || y >= height || team < 0 || team >= 6) {
            return false;
        }
        return board.board[y][x].team == team || board.board[y][x].team == -1;
    }
    public void move(int x, int y) {
        board.addTo(x, y, Host.players.get(cplayer).team.id);
        if (board.checkWinner() < 0) {
            cplayer ++;
            cplayer = cplayer % players;
        }
    }
}
