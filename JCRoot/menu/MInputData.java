package JCRoot.menu;

public class MInputData {
    public static final int EXITCODE = -1, NULLCODE = -2, ACTIONCODE = 0;
    public static final MInputData EXIT = new MInputData(EXITCODE), NULL = new MInputData(NULLCODE);
    public final int typeCode;
    public Object data = null;
    public MInputData(int typecode) {
        typeCode = typecode;
    }
    public MInputData(int typecode, Object data) {
        typeCode = typecode;
        this.data = data;
    }
    public boolean isNULL() {
        return typeCode == MInputData.NULLCODE;
    }
    public boolean isEXIT() {
        return typeCode == MInputData.EXITCODE;
    }
}
