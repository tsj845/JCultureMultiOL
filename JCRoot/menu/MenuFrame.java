package JCRoot.menu;

import java.util.LinkedHashMap;
import java.util.Scanner;

import JCRoot.game.Color;

import java.util.Map.Entry;

public class MenuFrame {
    public static final String DEFAULT = Color.RAW(251);
    public static final String TITLE = Color.RAW(249);
    // public static final String DISABLED = Color.RAW(245);
    public static final String DISABLED = Color.RAW(243);
    public static final String BACK_ITEM = Color.RAW(249);
    public static final String SYMBOLS = Color.RAW(246);
    //
    public static final String TOGGLE_NAME = Color.RAW(62);
    public static final String TOGGLE_OFF = Color.RAW(160);
    // public static final String TOGGLE_OFF = Color.RAW(124);
    // public static final String TOGGLE_ON = Color.RAW(28);
    // public static final String TOGGLE_ON = Color.RAW(30);
    public static final String TOGGLE_ON = Color.RAW(2);
    // public static final String TOGGLE_ON = Color.RAW(34);
    // public static final String TOGGLE_ON = Color.RAW(40);
    // public static final String TOGGLE_ON = Color.RAW(40), TOGGLE_OFF = Color.RAW(160), TOGGLE_NAME = Color.RAW(63);
    // public static final String NUMBER_NAME = Color.RAW(31);
    public static final String NUMBER_NAME = Color.RAW(32);
    public static final String NUMBER_VALUE = Color.RAW(38);
    // public static final String NUMBER_VALUE = Color.RAW(44), NUMBER_NAME = Color.RAW(33);
    public static final String GROUP_NAME = Color.RAW(97);
    // public static final String GROUP_NAME = Color.RAW(109);
    // public static final String GROUP_NAME = Color.RAW(220);
    // public static final String ACTION_NAME = Color.RAW(214);
    // public static final String ACTION_NAME = Color.RAW(34);
    public static final String ACTION_NAME = Color.RAW(71);
    private LinkedHashMap<String, ItemData> items = new LinkedHashMap<>();
    private boolean acceptNumbers = false;
    private String name;
    protected MenuFrame parent = null;
    public MenuFrame(String name) {this.name = name;}
    public MenuFrame setAcceptNumbers(boolean accept) {acceptNumbers = accept;return this;}
    private void setParent(MenuFrame p) {
        if (p == this) return;
        LinkedHashMap<String, ItemData> ni = new LinkedHashMap<>();
        ni.put("back", ItemData.SPECIAL_BACK);
        ni.putAll(items);
        items = ni;
        parent = p;
    }
    public MenuFrame addItem(String name, ItemData data) {
        if (data.itemType == ItemType.Group) {
            data.getGroup().setParent(this);
        }
        data.setName(name);
        items.put(name, data);
        return this;
    }
    private Entry<String, ItemData> resolveEntry(String input) {
        if (input.length() == 0) return null;
        if (acceptNumbers && input.matches("[0-9]+")) {
            int index = Integer.parseInt(input);
            if (index > 0 && index <= items.size()) {
                int i = 1;
                for (Entry<String, ItemData> entry : items.entrySet()) {
                    if (i == index) {
                        return entry;
                    }
                    i ++;
                }
            }
            return null;
        }
        for (Entry<String, ItemData> entry : items.entrySet()) {
            if (entry.getKey().equals(input)) {
                return entry;
            }
        }
        return null;
    }
    private MenuReturn process(String line) {
        Entry<String, ItemData> entry = resolveEntry(line);
        if (entry == null) return MenuReturn.MPROBLEM;
        ItemData val = entry.getValue();
        switch (val.itemType) {
            case Fake:return MenuReturn.MPASSIVE;
            case Toggle:return new MenuReturn(MenuReturn.ACTION, val);
            case Number:return new MenuReturn(MenuReturn.ACTION, val);
            case Group:return new MenuReturn(MenuReturn.FRAMEUPDATE, val.getGroup());
            case Action:return new MenuReturn(MenuReturn.ACTION, val);
            case Special: {
                if (val == ItemData.SPECIAL_BACK) {
                    return new MenuReturn(MenuReturn.FRAMEUPDATE, parent);
                }
                break;
            }
        }
        return MenuReturn.MPROBLEM;
    }
    public MenuReturn run(Scanner sc) {
        while (true) {
            System.out.println(this);
            System.out.print(" > ");
            String line = sc.nextLine();
            if (line.equalsIgnoreCase("done")) {
                return MenuReturn.MEXITSIGNAL;
            }
            MenuReturn mr = process(line);
            if (mr.code == MenuReturn.PROBLEM) {
                continue;
            }
            if (mr.code == MenuReturn.ACTION) {
                ItemData itemd = (ItemData)mr.data;
                if (itemd.itemType == ItemType.Toggle) {
                    itemd.toggleState();
                } else if (itemd.itemType == ItemType.Number) {
                    // to do
                }
            }
            return mr;
        }
    }
    private String generateText(String key, ItemData value) {
        String color = MenuFrame.DEFAULT;
        if (!value.isDisabled()) {
            switch (value.itemType) {
                case Special:{
                    if (value == ItemData.SPECIAL_BACK) {
                        color = MenuFrame.BACK_ITEM;
                    }
                    break;
                }
                case Fake:color=MenuFrame.DISABLED;break;
                case Toggle:color=MenuFrame.TOGGLE_NAME;break;
                case Group:color=MenuFrame.GROUP_NAME;break;
                case Number:color=MenuFrame.NUMBER_NAME;break;
                case Action:color=MenuFrame.ACTION_NAME;break;
            }
        } else {
            switch (value.itemType) {
                case Special:case Fake:color=MenuFrame.DISABLED;break;
                case Toggle:break;
                case Group:break;
                case Number:break;
                case Action:break;
            }
        }
        key = String.format("%s%s%s", color, key, MenuFrame.DEFAULT);
        switch (value.itemType) {
            case Special:case Fake:case Action:
                return key;
            case Toggle:
                // return String.format("%s%s: %s", key, MenuFrame.SYMBOLS, (value.getToggleState()?(MenuFrame.TOGGLE_ON+"yes"):(MenuFrame.TOGGLE_OFF+"no"))+MenuFrame.DEFAULT);
                return String.format("%s%s: %s", key, MenuFrame.SYMBOLS, (value.getToggleState()?(MenuFrame.TOGGLE_ON+"on"):(MenuFrame.TOGGLE_OFF+"off"))+MenuFrame.DEFAULT);
            case Group:
                return String.format("%s{%s%s}%s", MenuFrame.SYMBOLS, key, MenuFrame.SYMBOLS, MenuFrame.DEFAULT);
            case Number:
                return String.format("%s%s: %s%d%s", key, MenuFrame.SYMBOLS, MenuFrame.NUMBER_VALUE, value.getNumber(), MenuFrame.DEFAULT);
        }
        throw new Error("ENUM MATCH FAILURE");
    }
    public String toString() {
        // String f = MenuFrame.DEFAULT + " " + name + ":\n";
        String f = String.format(" %s%s%s:\n", MenuFrame.TITLE, name, MenuFrame.SYMBOLS);
        int itemnumber = 0;
        for (Entry<String, ItemData> entry : items.entrySet()) {
            String entrytext = generateText(entry.getKey(), entry.getValue());
            itemnumber ++;
            String c = entry.getValue().isDisabled() ? MenuFrame.DISABLED : MenuFrame.DEFAULT;
            if (acceptNumbers) {
                f += String.format("  %s(%d) %s\n", c, itemnumber, entrytext);
            } else {
                f += String.format("  %s- %s\n", c, entrytext);
            }
        }
        return f.substring(0, f.length()-1) + Color.DEFAULT;
    }
}
