/**@type {HTMLDivElement} */
const splashScreen = document.getElementById("splash");
/**@type {SVGSVGElement} */
const ssPlay = document.getElementById("ss-play-button");
/**@type {SVGSVGElement} */
const ssOpts = document.getElementById("ss-options-button");
/**@type {SVGSVGElement} */
const ssQuit = document.getElementById("ss-quit-button");

/**@type {HTMLDivElement} */
const optionsMenu = document.getElementById("options-modal");

ssPlay.children[0].addEventListener("click", ()=>{setScreen(1);});
ssOpts.children[0].addEventListener("click", ()=>{showModal(optionsMenu);});
ssQuit.children[0].addEventListener("click", ()=>{splashScreen.classList.add("S--quitconf");});

document.getElementById("ss-qc-no").addEventListener("click", ()=>{splashScreen.classList.remove("S--quitconf");});
document.getElementById("ss-qc-yes").addEventListener("click", ()=>{APP_EXIT();});

let currScreenId = 1;

/**
 * @param {0|1|2} sid
 * @returns {void}
 */
function setScreen(sid) {
    if (sid < 0 || sid > 2) return;
    if (sid === currScreenId) return;
    currScreenId = sid;
    document.body.classList.remove("screen-splash", "screen-servers", "screen-play");
    document.body.classList.add(["screen-splash", "screen-servers", "screen-play"][sid]);
}