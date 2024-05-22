package JCRoot;

import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.net.ServerSocket;
import java.net.Socket;
import java.util.Scanner;

public class Host {
    private static boolean usingPass = false;
    private static String hostname = "";
    private static String hostpass = "";
    private static Scanner sc = new Scanner(System.in);
    private static Thread servingThread = null;
    private static void cli() {
        while (true) {
            String line = sc.nextLine();
            if (line.equalsIgnoreCase("stop")) {
                System.out.println("STOPPING");
                return;
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
    private static void serve(Socket sock) throws IOException {
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
            if (usingPass) {
                sOut.write(1);
                if (!checkPassword(sock)) {
                    sock.close();
                    return;
                }
            } else {
                sOut.write(0);
            }
            sock.close();
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