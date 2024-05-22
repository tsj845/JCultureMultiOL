package JCRoot.game;

public class CColor {
    public final int red, green, blue;
    public CColor(int r, int g, int b) {
        red = r;
        green = g;
        blue = b;
    }
    public String toString() {
        return String.format("\u001b[38;2;%d;%d;%dm", red, green, blue);
    }
}
