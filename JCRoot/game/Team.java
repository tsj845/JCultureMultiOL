package JCRoot.game;

public class Team {
    public final int id;
    public final Color color;
    public final String name;
    public int pcount = 0;
    public int tscore = 0;
    public Team(int id, Color color, String name) {
        this.id = id;
        this.color = color;
        this.name = name;
    }
    public String toString() {
        return String.format("%s%s%s", color, name, Color.DEFAULT);
    }
}
