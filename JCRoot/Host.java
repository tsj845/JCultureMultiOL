package JCRoot;

import java.io.InputStream;
import java.io.OutputStream;
import java.net.InetSocketAddress;
import java.net.ServerSocket;
import java.net.Socket;
import java.nio.ByteBuffer;
import java.nio.channels.Pipe.SinkChannel;
import java.nio.channels.Pipe.SourceChannel;
import java.util.Scanner;
import java.util.TreeMap;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.LinkedBlockingDeque;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.TimeoutException;

import JCRoot.game.*;

public class Host {
    private static volatile int itcw=0, itch=0, itcp=0;
    private static boolean usingPass = false;
    private static String hostname = "";
    private static String hostpass = "";
    private static Scanner sc = new Scanner(System.in);
    public static TreeMap<Integer, Player> players = new TreeMap<>();
    public static TreeMap<Integer, Connection> connections = new TreeMap<>();
    private static Game game;
    private static LinkedBlockingDeque<Integer> comms = new LinkedBlockingDeque<>();
    private static Thread servingThread = null;
    private static CountDownLatch countdown = null;
    private static final Object SID_LOCK = new Object();
    private static volatile int CSID = 0;
    private static volatile int gamestate = 0;
    private static int getRestrictNum(String prompt, int lo, int hi) {
        while (true) {
            System.out.print(prompt);
            try {
                String l = sc.nextLine();
                if (l.equalsIgnoreCase("cancel")) return lo-1;
                int i = Integer.parseInt(l);
                if (i >= lo && i <= hi) {
                    return i;
                }
            } catch (Exception E) {}
            System.out.println("invalid");
            System.out.print(prompt);
        }
    }
    private static void backrunner() throws Exception {
        while (true) {
            int pcode = comms.takeFirst();
            Player p = players.get(pcode);
            p.pipe2.sink().write(ByteBuffer.wrap(new byte[]{0}));
            byte[] codebuf = new byte[1];
            p.pipe2.source().read(ByteBuffer.wrap(codebuf));
            if (gamestate == 0) {
                if (codebuf[0] == 0) {
                    System.out.printf("\"%s\" from %s%s%s has left\n", p.name, p.team.color, p.team.name, Color.DEFAULT);
                    synchronized(players) {
                        players.remove(pcode);
                        p.conn.s2O.write(0);
                        p.conn.s2O.flush();
                        p.conn.close(true);
                        countdown = new CountDownLatch(players.size());
                        for (Player player : players.values()) {
                            player.pipe.sink().write(ByteBuffer.wrap(new byte[]{4, (byte)(p.id)}));
                        }
                        countdown.await();
                    }
                }
            }
        }
    }
    private static void handle2(Player player) throws Exception {
        InputStream s2I = player.conn.s2I;
        while (true) {
            int dcode;
            try {dcode=read(player.conn, s2I);}
            catch (ClientGoneException CGE) {
                return;
            }
            if (dcode == 0) {
                comms.add(player.id);
                player.pipe2.source().read(ByteBuffer.wrap(new byte[1]));
                player.pipe2.sink().write(ByteBuffer.wrap(new byte[]{0}));
                return;
            }
        }
    }
    private static void runloop() throws Exception {
        while (true) {
            for (Player p : players.values()) {
                // SinkChannel snk = p.pipe.sink();
                OutputStream pOut = p.conn.s1O;
                if (p.id == game.cplayer) {
                    pOut.write(1);
                    // snk.write(ByteBuffer.wrap(new byte[]{1}));
                } else {
                    pOut.write(0);
                    // snk.write(ByteBuffer.wrap(new byte[]{0}));
                }
            }
            Player player = players.get(game.cplayer);
            InputStream pIn = player.conn.s1I;
            OutputStream pOut = player.conn.s1O;
            int x, y;
            while (true) {
                x = read(pIn);
                y = read(pIn);
                if (game.validate(x, y, player.team.id)) {
                    pOut.write(1);
                    break;
                } else {
                    System.out.println(player.team.id);
                    System.out.println(game.board.board[y][x].team);
                    pOut.write(0);
                }
            }
            // buf2[0] = (byte)x;
            // buf2[1] = (byte)y;
            // buf2[2] = (byte)player.team.id;
            // snk.write(ByteBuffer.wrap(buf2));
            // SourceChannel psrc = players.get(game.cplayer).pipe.source();
            // byte[] buf2 = new byte[3];
            // psrc.read(ByteBuffer.wrap(buf2));
            // game.move(buf2[0], buf2[1]);
            game.move(x, y);
            // System.out.println("RUNREAD:");
            // System.out.println(buf2);
            for (Player p : players.values()) {
                // SinkChannel snk = p.pipe.sink();
                // snk.write(ByteBuffer.wrap(new byte[]{2}));
                // snk.write(ByteBuffer.wrap(buf2));
                OutputStream sOut = p.conn.s1O;
                sOut.write(x);
                sOut.write(y);
                sOut.write(player.team.id);
            }
            if (game.board.checkWinner() != -2) {
                for (Player p : players.values()) {
                    SinkChannel snk = p.pipe.sink();
                    snk.write(ByteBuffer.wrap(new byte[]{3}));
                }
                System.out.printf("Team %s won!\n", Teams.teams[game.board.checkWinner()]);
                return;
            }
        }
    }
    private static void clcommand(String line) throws Exception {
        if (line.charAt(0) == '"') {
            System.out.println("unrecognized, try removing the quotes?");
            return;
        }
        if (line.equalsIgnoreCase("msg")) {
            System.out.println(
                "Welcome to JCultureMultiOL, the only (as of May 2024) online\n"+
                "  implementation of the game Culture, by Secret Lab\n"+
                "  if you don't know where to start, enter \"help\"\n"+
                "  have fun!"
            );
            return;
        }
        if (line.equalsIgnoreCase("list")) {
            for (Team team : Teams.teams) {
                System.out.print(team.id + ": ");
                System.out.println(team);
                for (Player p : players.values()) {
                    if (p.team.id == team.id) {
                        System.out.println("  " + p.name + "(" + p.id + ")");
                    }
                }
            }
        } else if (line.equalsIgnoreCase("setteam")) {
            clcommand("list");
            int pid;
            while (true) {
                try {
                    System.out.print("Enter player id: ");
                    String l = sc.nextLine();
                    if (l.equalsIgnoreCase("cancel")) return;
                    pid = Integer.parseInt(l);
                    if (pid < 0 || pid >= players.size()) {
                        System.out.println("out of range");
                        continue;
                    }
                    break;
                } catch (Exception E) {
                    System.out.println("malformed");
                }
            }
            int tid;
            while (true) {
                try {
                    System.out.print("Enter team id: ");
                    String l = sc.nextLine();
                    if (l.equalsIgnoreCase("cancel")) return;
                    tid = Integer.parseInt(l);
                    if (tid < 0 || tid > 5) {
                        System.out.println("out of range");
                        continue;
                    }
                    break;
                } catch (Exception E) {
                    System.out.println("malformed");
                }
            }
            Player pl = players.get(pid);
            Team nteam = Teams.teams[tid];
            System.out.printf("\"%s\" has switched from team %s%s%s to team %s%s%s\n", pl.name, pl.team.color, pl.team.name, Color.DEFAULT, nteam.color, nteam.name, Color.DEFAULT);
            pl.team = nteam;
            countdown = new CountDownLatch(players.size());
            for (Player p : players.values()) {
                p.pipe.sink().write(ByteBuffer.wrap(new byte[]{2, (byte)pid, (byte)tid}));
            }
            countdown.countDown();
            countdown.await();
        } else if (line.equalsIgnoreCase("help")) {
            System.out.println("HELP MENU");
            System.out.println(
                "- help    -- shows this menu\n"+
                "- msg     -- shows the welcome message\n"+
                "- stop    -- stops the server\n"+
                "- start   -- starts the game, will bring up a prompt for board dimensions\n"+
                "- list    -- lists all players in the session\n"+
                "- setteam -- switches what team a player is on,\n"+
                "              prompts for player id and team id,\n"+
                "              automatically runs the \"list\" command"
            );
        } else {
            System.out.println("unrecognized\n");
            clcommand("help");
        }
    }
    private static void cli() throws Exception {
        clcommand("msg");
        while (true) {
            String line = sc.nextLine();
            if (line.equalsIgnoreCase("stop")) {
                System.out.println("STOPPING");
                gamestate = -1;
                countdown = new CountDownLatch(players.size());
                for (Player p : players.values()) {
                    p.pipe.sink().write(ByteBuffer.wrap(new byte[]{0}));
                }
                countdown.await();
                return;
            } else if (line.equalsIgnoreCase("start")) {
                itcw = getRestrictNum("Enter width: ", 1, 26);
                if (itcw < 1) continue;
                itch = getRestrictNum("Enter height: ", 1, 26);
                if (itch < 1) continue;
                gamestate = 1;
                itcp = players.size();
                game = new Game(itcw, itch, itcp);
                countdown = new CountDownLatch(itcp);
                for (Player p : players.values()) {
                    p.pipe.sink().write(ByteBuffer.wrap(new byte[]{1}));
                }
                countdown.await();
                runloop();
                gamestate = 0;
            } else {
                clcommand(line);
            }
        }
    }
    private static void start(int port) {
        try (ServerSocket serv = new ServerSocket(port)) {
            Thread br = new Thread(){public void run(){try{Host.backrunner();}catch(Exception E){E.printStackTrace();}}};
            br.setDaemon(true);
            br.start();
            while (true) {
                Socket sock = serv.accept();
                Thread t = new Thread(){public void run(){try{serve(sock);}catch(Exception E){E.printStackTrace();}}};
                t.setDaemon(true);
                t.start();
            }
        } catch (Exception E) {
            // E.printStackTrace();
            // throw new IllegalStateException(E);
        }
    }
    private static boolean checkPassword(Socket sock) throws Exception {
        InputStream sIn = sock.getInputStream();
        OutputStream sOut = sock.getOutputStream();
        while (true) {
            byte[] bbuf = new byte[read(sIn)];
            read(sIn, bbuf);
            String passin = new String(bbuf);
            if (!passin.equals(hostpass)) {
                sOut.write(0);
                if (read(sIn) == 0) {
                    return false;
                }
                continue;
            }
            sOut.write(1);
            return true;
        }
    }
    private static void serve(Socket sock) throws Exception {
        InputStream sIn = sock.getInputStream();
        OutputStream sOut = sock.getOutputStream();
        int conncode = read(sIn);
        if (conncode == 0x66) {
            sOut.write(usingPass ? 1 : 0);
            sOut.write(hostname.length());
            sOut.write(hostname.getBytes());
            sock.close();
            return;
        }
        if (conncode == 0x22) {
            int gsid = (read(sIn)<<8) | read(sIn);
            if (connections.containsKey(gsid)) {
                Connection conn = connections.get(gsid);
                if (conn.s2 != null) {
                    sOut.write(0);
                    sock.close();
                    return;
                }
                conn.s2(sock);
                sOut.write(1);
                conn.cf1.complete(null);
                conn.cf2.get();
                handle2(players.get(conn.pid));
            } else {
                sOut.write(0);
                sock.close();
            }
            return;
        }
        if (conncode == 0x44) {
            // synchronized(Teams.teams) {
            //     if (Teams.teams.size() > 5) {
            //         sOut.write(2);
            //         sock.close();
            //         return;
            //     }
            // }
            if (gamestate != 0) {
                sOut.write(2);
                sock.close();
                return;
            }
            sOut.write(0);
            int sid;
            synchronized(SID_LOCK) {
                sid = CSID++;
            }
            Connection conn = new Connection(sid, sock, null);
            connections.put(sid, conn);
            sOut.write(sid >> 8);
            sOut.write(sid & 0xff);
            sOut.flush();
            try {
                conn.cf1.get(5, TimeUnit.SECONDS);
            } catch (TimeoutException T) {
                sock.close();
                connections.remove(sid);
                return;
            }
            if (usingPass) {
                sOut.write(1);
                if (!checkPassword(sock)) {
                    sock.close();
                    return;
                }
            } else {
                sOut.write(0);
            }
            byte[] bb = new byte[read(sIn)];
            read(sIn, bb);
            String cliname = new String(bb);
            // Color cc;
            int id = sid%256;
            // synchronized(Teams.teams) {
            //     id = Teams.teams.size();
            //     cc = new Color(id);
            //     Teams.teams.put(id, new Team(id, cc, cliname));
            // }
            sOut.write(id);
            sOut.write(id%6);
            conn.pid = id;
            Player p = new Player(id, Teams.teams[id%6], cliname, conn);
            System.out.printf("\"%s\" joined %s%s%s\n", p.name, p.team.color, p.team.name, Color.DEFAULT);
            preloop(p);
            // sOut.write(cc.red);
            // sOut.write(cc.green);
            // sOut.write(cc.blue);
        }
    }
    private static void preloop(Player player) throws Exception {
        OutputStream sOut = player.conn.s1O;
        synchronized(players) {
            for (Player p : players.values()) {
                Socket s = p.conn.s1;
                OutputStream pOut = s.getOutputStream();
                pOut.write(0x02);
                pOut.write(player.name.length());
                pOut.write(player.name.getBytes());
                pOut.write(player.id);
                pOut.write(player.team.id);
                sOut.write(0x02);
                sOut.write(p.name.length());
                sOut.write(p.name.getBytes());
                sOut.write(p.id);
                sOut.write(p.team.id);
            }
            players.put(player.id, player);
        }
        player.conn.cf2.complete(null);
        while (true) {
            byte[] bb = new byte[1];
            player.pipe.source().read(ByteBuffer.wrap(bb));
            // System.out.printf("PLAYER %d COMM %d\n", player.id, bb[0]);
            int pc = bb[0];
            if (pc == 0) {
                sOut.write(0);
                player.conn.close(true);
                countdown.countDown();
                return;
            }
            if (pc == 1) {
                sOut.write(1);
                sOut.write(itcw);
                sOut.write(itch);
                sOut.write(itcp);
                countdown.countDown();
                countdown.await();
                gameloop(player);
            }
            if (pc == 2) {
                byte[] buf = new byte[3];
                player.pipe.source().read(ByteBuffer.wrap(buf));
                sOut.write(3);
                sOut.write(buf[0]);
                sOut.write(buf[1]);
                countdown.countDown();
                countdown.await();
            }
            if (pc == 4) {
                byte[] buf = new byte[2];
                player.pipe.source().read(ByteBuffer.wrap(buf));
                sOut.write(4);
                sOut.write(buf[0]);
                countdown.countDown();
                countdown.await();
            }
        }
    }
    private static void gameloop(Player player) throws Exception {
        InputStream sIn = player.conn.s1I;
        OutputStream sOut = player.conn.s1O;
        SourceChannel src = player.pipe.source();
        SinkChannel snk = player.pipe.sink();
        while (true) {
            byte[] buf1 = new byte[1];
            byte[] buf2 = new byte[3];
            src.read(ByteBuffer.wrap(buf1));
            int comm = buf1[0];
            if (comm == 0) {
                sOut.write(0);
            } else if (comm == 1) {
                sOut.write(1);
                int x, y;
                while (true) {
                    x = read(sIn);
                    y = read(sIn);
                    if (game.validate(x, y, player.team.id)) {
                        sOut.write(1);
                        break;
                    } else {
                        System.out.println(player.team.id);
                        System.out.println(game.board.board[y][x].team);
                        sOut.write(0);
                    }
                }
                buf2[0] = (byte)x;
                buf2[1] = (byte)y;
                buf2[2] = (byte)player.team.id;
                snk.write(ByteBuffer.wrap(buf2));
            } else if (comm == 2) {
                src.read(ByteBuffer.wrap(buf2));
                sOut.write(buf2);
            } else if (comm == 3) {
                return;
            }
        }
    }
    private static int read(InputStream in) throws Exception {
        return read(null, in);
    }
    private static int read(Connection conn, InputStream in) throws Exception {
        int r = in.read();
        if (r < 0) crash(conn);
        return r;
    }
    private static int read(InputStream in, byte[] buf) throws Exception {
        return read(null, in, buf);
    }
    private static int read(Connection conn, InputStream in, byte[] buf) throws Exception {
        if (in.read(buf) != buf.length) crash(conn);
        return buf.length;
    }
    private static void crash(Connection conn) throws Exception {
        if (conn != null && conn.cgone()) throw new ClientGoneException();
        System.out.println();
        System.out.println("GAME CRASHED");
        System.out.println();
        throw new IllegalStateException();
    }
    public static void main(String[] args) throws Exception {
        Socket socket = new Socket();
        socket.connect(new InetSocketAddress("google.com", 80));
        System.out.println(socket.getLocalAddress());
        socket.close();
        System.out.print("Enter port: ");
        int port = Integer.parseInt(sc.nextLine());
        System.out.print("Enter name: ");
        hostname = sc.nextLine();
        System.out.print("use password? (y/N) ");
        if (sc.nextLine().toLowerCase().matches("(y|yes)")) {
            usingPass = true;
        }
        if (usingPass) {
            System.out.print("Enter password: ");
            hostpass = sc.nextLine();
        }
        servingThread = new Thread(){public void run(){Host.start(port);}};
        servingThread.setDaemon(true);
        servingThread.start();
        cli();
    }
}