package JCRoot;

import java.io.IOException;
import java.nio.channels.Pipe;

import JCRoot.game.Team;

public class Player {
    public int id;
    public String name;
    public Team team;
    public Connection conn;
    public Pipe pipe, pipe2;
    public Player(int id, Team team, String name) {
        this.id = id;
        this.team = team;
        this.name = name;
        this.conn = null;
        this.pipe = null;
    }
    public Player(int id, Team team, String name, Connection conn) throws IOException {
        this.id = id;
        this.team = team;
        this.name = name;
        this.conn = conn;
        this.pipe = Pipe.open();
        this.pipe2 = Pipe.open();
    }
}
