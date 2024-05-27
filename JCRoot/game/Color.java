package JCRoot.game;

public class Color {
    public static final String DEFAULT = "\u001b[38;5;7m\u001b[48;5;0m";
    public static final String TERMDEF = "\u001b[0m";
    public static final Color WHITE = new Color(6);
    public static final Color GRAY = new Color(-1);
    public final int n;
    public static void start() {
        Runtime.getRuntime().addShutdownHook(new Thread(){public void run(){System.out.println(Color.TERMDEF);}});
        System.out.print(DEFAULT);
    }
    public static String RAW(int code) {
        return new Color(code-9).toString();
    }
    public Color() {
        n = (int)(Math.random() * 6 + 9);
    }
    public Color(int n) {
        this.n = n+9;
    }
    public boolean equals(Object other) {
        return other instanceof Color && ((Color)other).n == n;
    }
    public String toString() {
        return String.format("\u001b[38;5;%dm", n);
    }
}
