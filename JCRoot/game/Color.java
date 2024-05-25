package JCRoot.game;

/*
 * function hslToRgb(h, s, l) {
  let r, g, b;

  if (s === 0) {
    r = g = b = l; // achromatic
  } else {
    const q = l < 0.5 ? l * (1 + s) : l + s - l * s;
    const p = 2 * l - q;
    r = hueToRgb(p, q, h + 1/3);
    g = hueToRgb(p, q, h);
    b = hueToRgb(p, q, h - 1/3);
  }

  return [round(r * 255), round(g * 255), round(b * 255)];
}

function hueToRgb(p, q, t) {
  if (t < 0) t += 1;
  if (t > 1) t -= 1;
  if (t < 1/6) return p + (q - p) * 6 * t;
  if (t < 1/2) return q;
  if (t < 2/3) return p + (q - p) * (2/3 - t) * 6;
  return p;
 */

public class Color {
    public static final String DEFAULT = "\u001b[38;5;7m\u001b[48;5;0m";
    public static final String TERMDEF = "\u001b[0m";
    public static final Color WHITE = new Color(6);
    public static final Color GRAY = new Color(-1);
    // public final int red, green, blue;
    public final int n;
    // private double hueToRgb(double h) {
    //     System.out.println(h);
    //     System.out.println("H2");
    //     if (h < 0.0d) h += 1.0d;
    //     if (h > 1.0d) h -= 1.0d;
    //     if (h < (1.0d/6.0d)) return h * 6.0d;
    //     if (h < (1.0d/2.0d)) return 1.0d;
    //     if (h < (2.0d/3.0d)) return 4.0d - (h * 6.0d);
    //     return 0.0d;
    // }
    public static void start() {
        Runtime.getRuntime().addShutdownHook(new Thread(){public void run(){System.out.println(Color.TERMDEF);}});
        System.out.print(DEFAULT);
    }
    public Color() {
        // double h = 0.25315662692529306;
        // // double h = Math.random();
        // System.out.println(h);
        // System.out.println("H1");
        // // double q = 1, p = 0;
        // double r=hueToRgb(h+(1.0d/3.0d)),g=hueToRgb(h),b=hueToRgb(h-(1.0d/3.0d));
        // System.out.println(r);
        // System.out.println(g);
        // System.out.println(b);
        // System.out.println("RGB1");
        // System.out.println(Math.round(r*255));
        // System.out.println(Math.round(g*255));
        // System.out.println(Math.round(b*255));
        // System.out.println("RGB2");
        // red = (int)Math.round(r*255);
        // green = (int)Math.round(g*255);
        // blue = (int)Math.round(b*255);
        // System.out.println(red);
        // System.out.println(green);
        // System.out.println(blue);
        // System.out.println("RGB3");
        n = (int)(Math.random() * 6 + 9);
    }
    public Color(int n) {
        this.n = n+9;
    }
    // public Color(int r, int g, int b) {
    //     red = r;
    //     green = g;
    //     blue = b;
    // }
    // public String toString() {
    //     return String.format("\u001b[38;2;%d;%d;%dm", red, green, blue);
    // }
    public String toString() {
        return String.format("\u001b[38;5;%dm", n);
    }
}
