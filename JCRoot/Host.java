package JCRoot;

import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.net.ServerSocket;
import java.net.Socket;
import java.nio.ByteBuffer;
import java.nio.channels.Pipe;
import java.nio.channels.Pipe.SinkChannel;
import java.nio.channels.Pipe.SourceChannel;
import java.util.Scanner;
import java.util.TreeMap;
import java.util.concurrent.CountDownLatch;

// import JCRoot.game.Color;
import JCRoot.game.Game;
import JCRoot.game.Team;
import JCRoot.game.Teams;

public class Host {
    private static volatile int itcw=0, itch=0, itcp=0;
    private static boolean usingPass = false;
    private static String hostname = "";
    private static String hostpass = "";
    private static Scanner sc = new Scanner(System.in);
    public static TreeMap<Integer, Player> players = new TreeMap<>();
    private static Game game;
    // private static LinkedBlockingDeque<Integer> comms = new LinkedBlockingDeque<>();
    private static Thread servingThread = null;
    private static CountDownLatch countdown = null;
    private static int getRestrictNum(String prompt, int lo, int hi) {
        while (true) {
            System.out.print(prompt);
            try {
                int i = Integer.parseInt(sc.nextLine());
                if (i >= lo && i <= hi) {
                    return i;
                }
            } catch (Exception E) {}
            System.out.println("invalid");
            System.out.print(prompt);
        }
    }
    private static void runloop() throws IOException {
        while (true) {
            for (Player p : players.values()) {
                // SinkChannel snk = p.pipe.sink();
                OutputStream pOut = p.sock.getOutputStream();
                if (p.id == game.cplayer) {
                    pOut.write(1);
                    // snk.write(ByteBuffer.wrap(new byte[]{1}));
                } else {
                    pOut.write(0);
                    // snk.write(ByteBuffer.wrap(new byte[]{0}));
                }
            }
            Player player = players.get(game.cplayer);
            InputStream pIn = player.sock.getInputStream();
            OutputStream pOut = player.sock.getOutputStream();
            int x, y;
            while (true) {
                x = pIn.read();
                y = pIn.read();
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
                OutputStream sOut = p.sock.getOutputStream();
                sOut.write(x);
                sOut.write(y);
                sOut.write(player.team.id);
            }
            if (game.board.checkWinner() != -1) {
                for (Player p : players.values()) {
                    SinkChannel snk = p.pipe.sink();
                    snk.write(ByteBuffer.wrap(new byte[]{3}));
                }
                System.out.printf("Team %s won!\n", Teams.teams[game.board.checkWinner()]);
                return;
            }
        }
    }
    private static void cli() throws IOException, InterruptedException {
        while (true) {
            String line = sc.nextLine();
            if (line.equalsIgnoreCase("stop")) {
                System.out.println("STOPPING");
                for (Player p : players.values()) {
                    p.pipe.sink().write(ByteBuffer.wrap(new byte[]{0}));
                }
                return;
            }
            if (line.equalsIgnoreCase("start")) {
                itcw = getRestrictNum("Enter width: ", 1, 26);
                itch = getRestrictNum("Enter height: ", 1, 26);
                itcp = players.size();
                game = new Game(itcw, itch, itcp);
                System.out.println(itcp);
                countdown = new CountDownLatch(itcp+1);
                for (Player p : players.values()) {
                    p.pipe.sink().write(ByteBuffer.wrap(new byte[]{1}));
                }
                countdown.countDown();
                countdown.await();
                runloop();
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
            }
            if (line.equalsIgnoreCase("setteam")) {
                int pid;
                while (true) {
                    try {
                        pid = Integer.parseInt(sc.nextLine());
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
                        tid = Integer.parseInt(sc.nextLine());
                        if (tid < 0 || tid > 5) {
                            System.out.println("out of range");
                            continue;
                        }
                        break;
                    } catch (Exception E) {
                        System.out.println("malformed");
                    }
                }
                players.get(pid).team = Teams.teams[tid];
                countdown = new CountDownLatch(players.size());
                for (Player p : players.values()) {
                    p.pipe.sink().write(ByteBuffer.wrap(new byte[]{2, (byte)pid, (byte)tid}));
                }
                countdown.countDown();
                countdown.await();
            }
        }
    }
    private static void start(int port) {
        try (ServerSocket serv = new ServerSocket(port)) {
            while (true) {
                Socket sock = serv.accept();
                Thread t = new Thread(){public void run(){try{serve(sock);}catch(Exception E){E.printStackTrace();}}};
                t.setDaemon(true);
                t.start();
            }
        } catch (Exception E) {
            E.printStackTrace();
        }
    }
    private static boolean checkPassword(Socket sock) throws IOException {
        InputStream sIn = sock.getInputStream();
        OutputStream sOut = sock.getOutputStream();
        while (true) {
            int plen = sIn.read();
            String passin = new String(sIn.readNBytes(plen));
            if (!passin.equals(hostpass)) {
                sOut.write(0);
                if (sIn.read() == 0) {
                    return false;
                }
                continue;
            }
            sOut.write(1);
            return true;
        }
    }
    private static void serve(Socket sock) throws IOException, InterruptedException {
        InputStream sIn = sock.getInputStream();
        OutputStream sOut = sock.getOutputStream();
        int conncode = sIn.read();
        if (conncode == 0x66) {
            sOut.write(usingPass ? 1 : 0);
            sOut.write(hostname.length());
            sOut.write(hostname.getBytes());
            sock.close();
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
            if (usingPass) {
                sOut.write(1);
                if (!checkPassword(sock)) {
                    sock.close();
                    return;
                }
            } else {
                sOut.write(0);
            }
            String cliname = new String(sIn.readNBytes(sIn.read()));
            // Color cc;
            int id = players.size();
            // synchronized(Teams.teams) {
            //     id = Teams.teams.size();
            //     cc = new Color(id);
            //     Teams.teams.put(id, new Team(id, cc, cliname));
            // }
            sOut.write(id);
            sOut.write(id);
            Player p = new Player(id, Teams.teams[id], cliname, sock, Pipe.open());
            preloop(sock, p);
            // sOut.write(cc.red);
            // sOut.write(cc.green);
            // sOut.write(cc.blue);
        }
    }
    private static void preloop(Socket sock, Player player) throws IOException, InterruptedException {
        OutputStream sOut = sock.getOutputStream();
        synchronized(players) {
            for (Player p : players.values()) {
                Socket s = p.sock;
                OutputStream pOut = s.getOutputStream();
                pOut.write(0x02);
                pOut.write(player.team.name.length());
                pOut.write(player.team.name.getBytes());
                pOut.write(player.id);
                pOut.write(player.team.id);
                sOut.write(0x02);
                sOut.write(p.team.name.length());
                sOut.write(p.team.name.getBytes());
                sOut.write(p.id);
                sOut.write(p.team.id);
            }
            players.put(player.id, player);
        }
        while (true) {
            byte[] bb = new byte[1];
            player.pipe.source().read(ByteBuffer.wrap(bb));
            // System.out.printf("PLAYER %d COMM %d\n", player.id, bb[0]);
            int pc = bb[0];
            if (pc == 0) {
                sOut.write(0);
                sock.close();
                return;
            }
            if (pc == 1) {
                sOut.write(1);
                sOut.write(itcw);
                sOut.write(itch);
                sOut.write(itcp);
                countdown.countDown();
                countdown.await();
                gameloop(sock, player);
            }
            if (pc == 2) {
                byte[] buf = new byte[2];
                player.pipe.source().read(ByteBuffer.wrap(buf));
                sOut.write(3);
                sOut.write(buf[0]);
                sOut.write(buf[1]);
                countdown.countDown();
                countdown.await();
            }
        }
    }
    private static void gameloop(Socket sock, Player player) throws IOException, InterruptedException {
        InputStream sIn = sock.getInputStream();
        OutputStream sOut = sock.getOutputStream();
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
                    x = sIn.read();
                    y = sIn.read();
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
    public static void main(String[] args) throws Exception {
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