package JCRoot;

import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.net.InetAddress;
import java.net.Socket;
import java.util.Scanner;

public class Client {
    private static Scanner sc = new Scanner(System.in);
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
            System.out.printf("Confirm joining \"%s\" with%s password? (y/N) ", hname, hasPass?"":"out");
            if (sc.nextLine().toLowerCase().matches("(y|yes)")) {
                return true;
            }
        } catch (Exception E) {
            E.printStackTrace();
        }
        return false;
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
            OutputStream sOut = sock.getOutputStream();
            InputStream sIn = sock.getInputStream();
            sOut.write(0x44);
            if (sIn.read() > 0) {
                if (!passwordCheck(sock)) {
                    sock.close();
                    return;
                }
            }
            sock.close();
        } catch (Exception E) {
            E.printStackTrace();
        }
    }
    public static void main(String[] args) throws Exception {
        start(InetAddress.getByName(args[0]), Integer.parseInt(args[1]));
    }
}
