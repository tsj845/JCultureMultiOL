const modalStack = [];

/**
 * checks if the given element is actually a modal
 * @param {HTMLElement} modal element to check
 * @returns {boolean}
 */
function isModal(modal) {
    if (!modal instanceof HTMLElement) {
        return false;
    }
    return modal.nodeName === "DIV" && modal.classList.contains("modal-back");
}

/**
 * shows a modal
 * @param {ModalElement} modal the modal to show
 * @returns {void}
 * @throws {TypeError} if the given node was not a modal
 */
function showModal(modal) {
    if (!isModal(modal)) {
        throw new TypeError("must be modal");
    }
    modal.classList.add("open");
    modalStack.push(modal);
}

/**
 * hides a modal
 * @param {ModalElement} modal the modal to hide
 * @returns {void}
 * @throws {TypeError} if the given node was not a modal
 */
function hideModal(modal) {
    if (!isModal(modal)) {
        throw new TypeError("must be modal");
    }
    modal.classList.remove("open");
    if (modalStack.includes(modal)) {
        modalStack.splice(modalStack.indexOf(modal), 1);
    }
}

document.addEventListener("keyup", (e) => {
    let k = e.code.toString();
    if (k === "Escape") {
        if (modalStack.length > 0) {
            hideModal(modalStack[modalStack.length-1]);
        }
    }
});