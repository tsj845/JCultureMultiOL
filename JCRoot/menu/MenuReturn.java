package JCRoot.menu;

public class MenuReturn {
    public static final int PASSIVE = 0, FRAMEUPDATE = 1, ACTION = 2, EXITSIGNAL = 3, PROBLEM = 4;
    public static final MenuReturn MPASSIVE = new MenuReturn(PASSIVE, null), MEXITSIGNAL = new MenuReturn(EXITSIGNAL, null), MPROBLEM = new MenuReturn(PROBLEM, null);
    public final int code;
    public Object data;
    public MenuReturn(int code, Object data) {
        this.code = code;
        this.data = data;
    }
}
