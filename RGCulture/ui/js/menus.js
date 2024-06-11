/**
 * @typedef MItemType_t
 * @type {{Toggle:number,SubMenu:number,Next:number,Prev:number}}
 */

/**
 * @enum
 * @type {MItemType_t}
 */
const MItemType = frozen({
    Toggle:0,
    SubMenu:1,
    Next:2,
    Prev:3,
});

class MInputMethod {
    /**
     * @param {MItemType_t} type
     */
    constructor(type) {
        /**@type {MItemType_t} */
        this.type = type;
        switch(type) {
            case MItemType.Next:
                break;
            case MItemType.Prev:
                break;
            case MItemType.SubMenu:
                break;
            case MItemType.Toggle:
                break;
        }
    }
}

class MenuItem {
    /**
     * makes a new menu item
     * @param {MItemType_t} type
     * @param {string} name
     * @param {any} prop
     */
    constructor(type, name, prop) {
        /**@type {MItemType_t} */
        this.type = type;
        this.prop = prop;
        /**@type {string} */
        this.name = name;
        /**@type {MInputMethod} */
        this.elem = null;
    }
    bindTo() {}
}