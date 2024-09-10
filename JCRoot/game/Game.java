package JCRoot.game;

import java.io.FileOutputStream;
import java.io.IOException;
import java.util.ArrayList;
import java.util.stream.Collectors;

import JCRoot.Host;
import JCRoot.Player;

public class Game {
    public final int width, height;
    public final Board board;
    public ArrayList<Integer> plist;
    // public int[] plist = Host.players.navigableKeySet().stream().mapToInt(e->e).toArray();
    public int pindex = 0;
    // public int cplayer = plist[pindex];
    public int cplayer;
    private int round = 0;
    private final int botinter;
    public Game(int w, int h, int bi) {
        plist = new ArrayList<>(Host.players.navigableKeySet());
        cplayer = plist.get(pindex);
        width = w;
        height = h;
        board = new Board(w, h);
        botinter = bi;
    }
    public boolean canMove(Player player) {
        int team = player.team.id;
        if (player.team.botteam) {
            if (round % botinter == 0) {
                return true;
            } else {
                return false;
            }
        }
        if (team > 5) return false;
        for (int y = 0; y < height; y ++) {
            for (int x = 0; x < width; x ++) {
                if (board.board[y][x].team == -1 || board.board[y][x].team == team) {
                    return true;
                }
            }
        }
        return false;
    }
    public boolean validate(int x, int y, int team) {
        if (x < 0 || y < 0 || x >= width || y >= height || team < 0 || team >= 6) {
            return false;
        }
        if (Teams.teams[team].botteam) {
            return true;
        }
        return board.board[y][x].team == team || board.board[y][x].team == -1;
    }
    public boolean move(int x, int y) {
        board.addTo(x, y, Host.players.get(cplayer).team.id);
        boolean r = false;
        if (board.checkWinner() < 0) {
            do {
                pindex ++;
                // pindex = pindex % plist.length;
                // cplayer = plist[pindex];
                pindex = pindex % plist.size();
                cplayer = plist.get(pindex);
                if (pindex == 0) {
                    r = true;
                    round ++;
                }
            } while (!canMove(Host.players.get(cplayer)));
        }
        return r;
    }
    public void save(String loc) {
        try (FileOutputStream fOut = new FileOutputStream(loc)) {
            fOut.write(width);
            fOut.write(height);
            for (Team team : Teams.teams) {
                fOut.write(team.tscore);
            }
            for (int y = 0; y < height; y ++) {
                for (int x = 0; x < width; x ++) {
                    fOut.write(board.board[y][x].team);
                    fOut.write(board.board[y][x].value);
                }
            }
        } catch (IOException IOE) {
            IOE.printStackTrace();
        }
    }
}
