package JCRoot.game;

public class Color {
    public static final String DEFAULT = "\u001b[38;5;7m\u001b[48;5;0m";
    public static final String TERMDEF = "\u001b[0m";
    public static final Color WHITE = new Color(6);
    // public static final Color GRAY = new Color(-1);
    public static final String GRAY = Color.RAW(8)+"\u001b[48;5;0m";
    public static final String HC_BACKGROUND = "\u001b[48;5;235m"; // high contrast background color
    public static final String HIGHLIGHT = "\u001b[48;5;238m"; // highlight background color
    public static final String[] NORMAL_COLORS = new String[]{
        Color.RAW(8),
        Color.RAW(160),
        // Color.RAW(21),
        Color.RAW(27),
        // Color.RAW(40),
        Color.RAW(34),
        // Color.RAW(220),
        Color.RAW(214),
        // Color.RAW(128),
        Color.RAW(129),
        // Color.RAW(126),
        // Color.RAW(81)
        Color.RAW(39)
    };
    public static final String[] VOLATILE_COLORS = new String[]{
        Color.RAW(249),
        Color.RAW(196),
        // Color.RAW(33),
        Color.RAW(69),
        Color.RAW(46),
        // Color.RAW(11),
        Color.RAW(226),
        // Color.RAW(220),
        // Color.RAW(13),
        Color.RAW(201),
        // Color.RAW(14)
        Color.RAW(51)
    };
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
    public static void main(String[] args) {
        start();
        for (String[] colors : new String[][]{NORMAL_COLORS, VOLATILE_COLORS}) {
            for (String color : colors) {
                System.out.printf("%s█ ", color);
            }
            System.out.println(DEFAULT);
        }
    }
}
