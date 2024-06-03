/**@type {{modal:HTMLDivElement,field:HTMLInputElement,button:HTMLInputElement}} */
const pwModal = {modal:document.getElementById("password-modal")};
pwModal.field = pwModal.modal.children[0].children[0];
pwModal.button = pwModal.modal.children[0].children[1];

/**@type {{pid:number,tid:number}} */
const gamedata = {};

let DEBUG_MODE = false;

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

window.addEventListener("load", async () => {
    emit("window-ready", "");
    DEBUG_MODE = await invoke("get_is_debug", "");
    if (DEBUG_MODE) {
        console.log("DEBUG MODE");
    }
    initBoard(10, 10);
});