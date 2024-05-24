package JCRoot;

import java.nio.channels.Pipe;

import JCRoot.game.Team;

public class Player {
    public int id;
    public String name;
    public Team team;
    public Connection conn;
    public Pipe pipe;
    public Player(int id, Team team, String name) {
        this.id = id;
        this.team = team;
        this.name = name;
        this.conn = null;
        this.pipe = null;
    }
    public Player(int id, Team team, String name, Connection conn, Pipe pipe) {
        this.id = id;
        this.team = team;
        this.name = name;
        this.conn = conn;
        this.pipe = pipe;
    }
}
