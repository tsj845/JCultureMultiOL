package JCRoot;

import java.net.Socket;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.util.concurrent.CompletableFuture;

public class Connection {
    public final int sid;
    public Socket s1, s2;
    public InputStream s1I, s2I;
    public OutputStream s1O, s2O;
    public CompletableFuture<Boolean> cf = new CompletableFuture<>();
    public Connection(int sid, Socket s1, Socket s2) throws IOException {
        this.sid = sid;
        this.s1 = s1;
        this.s2 = s2;
        this.s1I = s1.getInputStream();
        this.s1O = s1.getOutputStream();
        if (s2 != null) {
            this.s2I = s2.getInputStream();
            this.s2O = s2.getOutputStream();
        }
    }
    public void s2(Socket s2) throws IOException {
        this.s2 = s2;
        this.s2I = s2.getInputStream();
        this.s2O = s2.getOutputStream();
    }
}
