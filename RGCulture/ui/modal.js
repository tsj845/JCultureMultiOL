/**
 * @typedef ModalElement
 * @type {HTMLDivElement}
 */

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
}