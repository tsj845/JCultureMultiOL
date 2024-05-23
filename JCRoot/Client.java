package JCRoot;

import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.net.InetAddress;
import java.net.Socket;
import java.util.Scanner;
import java.util.TreeMap;

import JCRoot.game.Board;
import JCRoot.game.Color;
// import JCRoot.game.Team;
import JCRoot.game.Teams;

public class Client {
    private static int pnum = -1;
    private static TreeMap<Integer, Player> players = new TreeMap<>();
    private static Scanner sc = new Scanner(System.in);
    private static Board board;
    private static boolean confirm(InetAddress addr, int port) {
        try (Socket sock = new Socket(addr, port)) {
            OutputStream sOut = sock.getOutputStream();
            InputStream sIn = sock.getInputStream();
            sOut.write(0x66);
            boolean hasPass = sIn.read() > 0;
            byte[] buf = new byte[sIn.read()];
            sIn.read(buf);
            sock.close();
            String hname = new String(buf);
            System.out.printf("Confirm joining \"%s\" with%s password? (Y/n) ", hname, hasPass?"":"out");
            if (sc.nextLine().toLowerCase().matches("(n|no)")) {
                return false;
            }
        } catch (Exception E) {
            E.printStackTrace();
        }
        return true;
    }
    private static boolean passwordCheck(Socket sock) throws IOException {
        OutputStream sOut = sock.getOutputStream();
        InputStream sIn = sock.getInputStream();
        while (true) {
            System.out.print("Enter password: ");
            String pass = sc.nextLine();
            sOut.write(pass.length());
            sOut.write(pass.getBytes());
            if (sIn.read() == 0) {
                System.out.println("INCORRECT PASSWORD");
                System.out.print("Try again? (Y/n) ");
                if (sc.nextLine().toLowerCase().matches("(n|no)")) {sOut.write(0);return false;}
                sOut.write(1);
                continue;
            }
            System.out.println("PASSWORD ACCEPTED");
            return true;
        }
    }
    private static void start(InetAddress addr, int port) {
        if (!confirm(addr, port)) return;
        try (Socket sock = new Socket(addr, port)) {
            InputStream sIn = sock.getInputStream();
            OutputStream sOut = sock.getOutputStream();
            sOut.write(0x44);
            int c = sIn.read();
            if (c == 2) {sock.close();return;}
            if (c > 0) {
                if (!passwordCheck(sock)) {
                    sock.close();
                    return;
                }
            }
            System.out.print("Enter nickname: ");
            String clname = sc.nextLine();
            sOut.write(clname.length());
            sOut.write(clname.getBytes());
            pnum = sIn.read();
            int teamid = sIn.read();
            // Team team = new Team(teamid, new Color(teamid), clname);
            // Teams.teams.put(teamid, team);
            players.put(pnum, new Player(pnum, Teams.teams[teamid], clname));
            preloop(sock);
        } catch (Exception E) {
            E.printStackTrace();
        }
    }
    private static void preloop(Socket sock) throws IOException {
        InputStream sIn = sock.getInputStream();
        while (true) {
            int commcode = sIn.read();
            if (commcode == 0) {
                System.out.println("THE HOST ENDED THE SESSION");
                sock.close();
                return;
            }
            if (commcode == 1) {
                System.out.println("HOST HAS STARTED THE GAME");
                board = new Board(sIn.read(), sIn.read(), sIn.read());
                System.out.println(board.w);
                System.out.println(board.h);
                System.out.println(board.p);
                gameloop(sock);
            }
            if (commcode == 2) {
                String otname = new String(sIn.readNBytes(sIn.read()));
                int pid = sIn.read();
                int tid = sIn.read();
                // Team team = new Team(tid, new Color(tid), otname);
                // Teams.teams.put(tid, team);
                players.put(pid, new Player(pid, Teams.teams[tid], otname));
            }
            if (commcode == 3) {
                int pid = sIn.read();
                int tid = sIn.read();
                players.get(pid).team = Teams.teams[tid];
            }
        }
    }
    private static void gameloop(Socket sock) throws IOException {
        InputStream sIn = sock.getInputStream();
        OutputStream sOut = sock.getOutputStream();
        while (true) {
            System.out.println(board);
            if (board.checkWinner() != -1) {
                System.out.printf("Team %s has won!\n", Teams.teams[board.checkWinner()]);
                return;
            }
            int ccode = sIn.read();
            if (ccode == 1) {
                int row;
                int col;
                while (true) {
                    System.out.printf("%sEnter Move:%s\n", players.get(pnum).team.color, Color.DEFAULT);
                    String l = sc.nextLine().toUpperCase();
                    if (l.length() == 0) {
                        System.out.println("malformed");
                        continue;
                    }
                    col = ((int)l.charAt(0)) - ((int)'A');
                    if (col < 0 || col >= board.w) {
                        System.out.println("malformed");
                        continue;
                    }
                    if (l.substring(1).matches("^[0-9]{1,2}$")) {
                        row = Integer.parseInt(l.substring(1))-1;
                    } else {
                        System.out.println("malformed");
                        continue;
                    }
                    if (row < 0 || row >= board.h) {
                        System.out.println("malformed");
                        continue;
                    }
                    sOut.write(col);
                    sOut.write(row);
                    if (sIn.read() == 0) {
                        System.out.println("invalid");
                        continue;
                    }
                    break;
                }
            }
            int col = sIn.read();
            int row = sIn.read();
            int team = sIn.read();
            board.addTo(col, row, team);
        }
    }
    public static void main(String[] args) throws Exception {
        start(InetAddress.getByName(args[0]), Integer.parseInt(args[1]));
    }
}
