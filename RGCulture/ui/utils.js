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