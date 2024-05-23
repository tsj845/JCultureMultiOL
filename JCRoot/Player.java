package JCRoot;

import java.net.Socket;
import java.nio.channels.Pipe;

import JCRoot.game.Team;

public class Player {
    public int id;
    public String name;
    public Team team;
    public Socket sock;
    public Pipe pipe;
    public Player(int id, Team team, String name) {
        this.id = id;
        this.team = team;
        this.name = name;
        this.sock = null;
        this.pipe = null;
    }
    public Player(int id, Team team, String name, Socket sock, Pipe pipe) {
        this.id = id;
        this.team = team;
        this.name = name;
        this.sock = sock;
        this.pipe = pipe;
    }
}
