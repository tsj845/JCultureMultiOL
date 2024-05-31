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

/**@type {{modal:HTMLDivElement,field:HTMLInputElement,button:HTMLInputElement}} */
const pwModal = {modal:document.getElementById("password-modal")};
pwModal.field = pwModal.modal.children[0].children[0];
pwModal.button = pwModal.modal.children[0].children[1];

/**@type {{pid:number,tid:number}} */
const gamedata = {};

pwModal.button.addEventListener("click", () => {
    emit("password-input", pwModal.field.value);
});

listen("password-accept", (_) => {
    hideModal(pwModal.modal);
    pwModal.field.value = "";
});

listen("password-reject", (_) => {
    pwModal.field.value = "";
});

listen("prompt-password", (_) => {
    showModal(pwModal.modal);
});

window.addEventListener("load", () => {
    emit("window-ready", "");
});