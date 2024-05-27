package JCRoot.menu;

public class ItemData {
    public final ItemType itemType;
    private boolean toggleState = false;
    private int numvalue = 0;
    private Integer nummin = null;
    private Integer nummax = null;
    private MenuFrame subgroup = null;
    private boolean disabled = false;
    private String name = null;
    public int iid = -1;
    private ItemData(ItemType type) {itemType=type;}
    private ItemData(int special_code) {itemType=ItemType.Special;numvalue=special_code;}
    /**
     * <ul>
     * <li>creates a "fake" item</li>
     * <li>"fake" items are items without any data</li>
     * <li>"fake" items will always return true through <code>isDisabled</code>, regardless of what is passed to <code>setDisabled</code></li>
     * </ul>
     */
    public static ItemData Fake() {return new ItemData(ItemType.Fake);}
    public static ItemData Action() {return new ItemData(ItemType.Action);}
    public static ItemData Group(MenuFrame group) {
        ItemData r = new ItemData(ItemType.Group);
        r.subgroup = group;
        return r;
    }
    public static ItemData Toggle(boolean defaultV) {
        ItemData r = new ItemData(ItemType.Toggle);
        r.toggleState = defaultV;
        return r;
    }
    public static ItemData Number(int defaultV, Integer min, Integer max) {
        ItemData r = new ItemData(ItemType.Number);
        r.numvalue = defaultV;
        r.nummin = min;
        r.nummax = max;
        return r;
    }
    public static ItemData Number(int defaultV) {
        return Number(defaultV, null, null);
    }
    public ItemData withIID(int iid) {
        this.iid = iid;
        return this;
    }
    public boolean checkNumRange(int test) {
        if (itemType != ItemType.Number) throw new IllegalStateException("cannot check range on non-numeric option");
        if (nummin != null && test < nummin) return false;
        if (nummax != null && test > nummax) return false;
        return true;
    }
    public int getNumber() {
        if (itemType != ItemType.Number) throw new IllegalStateException("cannot get numeric value of non-numeric option");
        return numvalue;
    }
    public boolean getToggleState() {
        if (itemType != ItemType.Toggle) throw new IllegalStateException("cannot get toggle state of non-toggle option");
        return toggleState;
    }
    public void setToggleState(boolean state) {
        if (itemType != ItemType.Toggle) throw new IllegalStateException("cannot set toggle state of non-toggle option");
        toggleState = state;
    }
    public void toggleState() {
        if (itemType != ItemType.Toggle) throw new IllegalStateException("cannot toggle state of non-toggle option");
        toggleState = !toggleState;
    }
    public MenuFrame getGroup() {
        if (itemType != ItemType.Group) throw new IllegalStateException("cannot get group of non-group option");
        return subgroup;
    }
    public boolean isDisabled() {
        if (itemType == ItemType.Fake) return true;
        return disabled;
    }
    public void setDisabled(boolean disabled) {
        this.disabled = disabled;
    }
    protected void setName(String name) {
        this.name = name;
    }
    protected String getName() {
        return name;
    }
    protected static final ItemData SPECIAL_BACK = new ItemData(0);
}
