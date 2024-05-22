package JCRoot.game;

public class Team {
    public final int id;
    public final Color color;
    public final String name;
    public Team(int id, Color color, String name) {
        this.id = id;
        this.color = color;
        this.name = name;
    }
    public String toString() {
        return String.format("%s%s%s", color, name, Color.DEFAULT);
    }
}
