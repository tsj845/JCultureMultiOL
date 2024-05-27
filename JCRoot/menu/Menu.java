package JCRoot.menu;

import java.util.Scanner;

import JCRoot.game.Color;

public class Menu {
    public static final int TOP = 0;
    private final MenuFrame topFrame;
    private MenuFrame cframe;
    public Menu(MenuFrame top) {
        topFrame = top;
        cframe = top;
    }
    public MInputData run(Scanner sc) {
        MenuReturn ret = cframe.run(sc);
        if (ret.code == MenuReturn.PASSIVE) {
            return MInputData.NULL;
        }
        if (ret.code == MenuReturn.FRAMEUPDATE) {
            cframe = (MenuFrame)ret.data;
            return MInputData.NULL;
        }
        if (ret.code == MenuReturn.EXITSIGNAL) {
            return MInputData.EXIT;
        }
        if (ret.code == MenuReturn.ACTION) {
            return new MInputData(MInputData.ACTIONCODE, ret.data);
        }
        throw new IllegalStateException("BAD MENU RETURN CODE");
    }
    public void setState(int statecode) {
        if (statecode == Menu.TOP) cframe = topFrame;
    }
    public static void main(String[] args) {
        Color.start();
        MenuFrame root = new MenuFrame("ROOT");
        MenuFrame top = new MenuFrame("TEST");
        root.addItem("TEST", ItemData.Group(top));
        top.addItem("tDon", ItemData.Toggle(true));
        top.addItem("tDoff", ItemData.Toggle(false));
        top.addItem("number", ItemData.Number(0));
        top.addItem("fake", ItemData.Fake());
        top.addItem("group", ItemData.Group(top));
        top.addItem("action", ItemData.Action());
        top.setAcceptNumbers(true);
        Menu m = new Menu(top);
        if (args.length > 0 && args[0].equalsIgnoreCase("colors")) {
            System.out.println(m.topFrame);
            return;
        }
        Scanner sc = new Scanner(System.in);
        while (true) {
            MInputData mid = m.run(sc);
            if (mid.isEXIT()) break;
        }
        sc.close();
    }
}
