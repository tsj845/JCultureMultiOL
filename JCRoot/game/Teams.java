package JCRoot.game;

// import java.util.TreeMap;

public class Teams {
    // public static TreeMap<Integer, Team> teams = new TreeMap<>();
    public static Team[] teams = new Team[]{
        new Team(0, new Color(0), "red"),
        new Team(1, new Color(18), "blue"),
        new Team(2, new Color(1), "green"),
        new Team(3, new Color(2), "yellow"),
        new Team(4, new Color(4), "magenta"),
        new Team(5, new Color(5), "cyan")
    };
    public static void reset() {
        for (Team team : teams) {
            team.pcount = 0;
            team.tscore = 0;
        }
    }
}
