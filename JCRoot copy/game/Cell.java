package JCRoot.game;

public class Cell {
    public int x, y;
    public int value;
    public int team;
    public Cell(int x, int y, int v, int t) {
        this.x = x;
        this.y = y;
        value = v;
        team = t;
    }
}
