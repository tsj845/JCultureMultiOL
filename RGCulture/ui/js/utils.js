/**
 * @template T
 * @param {T} o object to freeze
 * @returns {Readonly<T>}
 */
function frozen(o) {
    if (typeof o === 'object') {
        for (const key in o) {
            o[key] = frozen(o[key]);
        }
    }
    return Object.freeze(o);
}

/*
 * using these cursed wrappers b/c that's the only way
 * to get my linter to work properly
 */

/**
 * @typedef TauriEvent
 * @type {{event:String,payload:Object}}
 */
/**
 * @param {String} name
 * @param {Object} data
 * @returns {Promise<Object>}
 */
const invoke = (name,data)=>{return window.__TAURI__.invoke(name,data);};
/**
 * @param {String} name
 * @param {Object} data
 * @returns {void}
 */
const emit = (name,data)=>{window.__TAURI__.event.emit(name,data);};
/**
 * @param {String} name
 * @param {(event:TauriEvent)=>void} callback
 * @returns {Promise<()=>void>}
 */
const listen = (name,callback)=>{return window.__TAURI__.event.listen(name,callback);};
/**
 * @param {String} name
 * @param {(event:TauriEvent)=>void} callback
 * @returns {void}
 */
const once = (name,callback)=>{window.__TAURI__.event.once(name,callback);};

function APP_EXIT() {
    emit("close", "");
}

let LINK = document.createElement("link");
LINK.href = "tileimg.css";
LINK.rel = "stylesheet";
LINK.type = "text/css";

/**
 * finds the correct display for the given value and returns it
 * @param {number} value
 * @returns {HTMLElement}
 */
function produceBoardDisplay(value) {
    let tset = atilesets[current_set[0]].sets[current_set[1]];
    /**@type {HTMLElement} */
    let elem;
    if (tset.isText) {
        elem = document.createElement("span");
        elem.textContent = tset.set[value-1];
    } else {
        // let elem = document.createElement("embed");
        elem = document.createElement("iframe");
        /**@type {Document} */
        let d = imageTilesCache[`${tset.set}${tset.ext}`][value-1];
        /**@type {Element} */
        let c = d.firstElementChild;
        // elem.contentDocument = d.cloneNode(true);
        // elem.append(d.cloneNode(true));
        elem.addEventListener("load",()=>{
            elem.contentDocument.body.append(c.cloneNode(true));
            elem.contentDocument.head.append(LINK.cloneNode(true));
            // elem.contentDocument.write(c.outerHTML);
            // elem.contentDocument.firstElementChild.replaceChildren(c.cloneNode(true));
        });
        // let doc = elem.contentWindow.document;
        // doc.open();
        // doc.appendChild(c.cloneNode(true));
        // doc.close();
        // elem.replaceChildren(c.cloneNode(true));
        // elem = document.createElement("img");
        // elem.src = `${tset.set}${value}${tset.ext}`;
    }
    elem.classList.add("board-space-text");
    return elem;
}

function resolveColor(x, y, team) {
    return normal_colors[team];
}

/**
 * sets the team of a board space
 * @param {number} x
 * @param {number} y
 * @param {HTMLElement} elem
 * @param {number} team
 * @returns {void}
 */
function setBoardDisplayTeam(x, y,team) {
    const elem = gameBoard.children[y*playBoard.w+x].children[0];
    if (elem.nodeName === "IFRAME") {
        /**@type {HTMLIFrameElement} */
        let e = elem;
        const f = () => {
            const g = () => {
                let s = e.contentDocument.body.querySelector("svg");
                s.style.cssText = `--color:${resolveColor(x, y, team)}`;
            };
            if (!e.contentDocument.readyState === "complete") {
                e.contentWindow.addEventListener("load", g);
            } else {
                g();
            }
        }
        if (e.contentDocument === null) {
            e.addEventListener("load", f);
        } else {
            f();
        }
    } else {
        elem.style.cssText = `--color:${resolveColor(x, y, team)}`;
    }
}

/**
 * creates an element with the given attributes
 * @param {string} type
 * @param {{classList?:string[],textContent?:string,cssText?:string}} attrs
 * @returns {HTMLElement}
 */
function createElementA(type, attrs) {
    const e = document.createElement(type);
    for (const attr in attrs) {
        switch(attr) {
            case "classList":
                e.classList.add(...attrs["classList"]);
                break;
            case "textContent":
                e.textContent = attrs["textContent"];
                break;
            case "cssText":
                e.style.cssText = attrs["cssText"];
                break;
        }
    }
    return e;
}
