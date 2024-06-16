/**@type {HTMLDivElement} */
const DOMServerList = document.getElementById("server-list");

document.getElementById("sl-back-button").children[0].addEventListener("click", () => {setScreen(0);setActiveCard(null);});

/**@type {HTMLDivElement} */
const DOMServerControls = document.getElementById("server-controls");

listen("join-server-failed", (ev)=>{console.log(ev.payload);});
DOMServerControls.children[0].addEventListener("click", () => {
    console.log(currActiveCard);
    if (currActiveCard !== null) {
        const cid = getCardId(currActiveCard);
        /**@type {ServerCardData} */
        const scd = serverCardMap[cid][0];
        /**@type {string} */
        let ad = scd["addr"];
        once("confirm-server", (_)=>{console.log("CONFIRMING");emit("server-confirmed", "true");});
        emit("join-server", {"cdat":{"addr":ad.slice(0, ad.indexOf(":")),"port":Number(ad.slice(ad.indexOf(":")+1))},"name":"TR1"});
    }
});

const serverCardMap = {};

/**
 * @param {HTMLElement} elem
 * @returns {boolean}
 */
function isServerCard(elem) {
    if (!elem instanceof HTMLElement) {
        return false;
    }
    return elem.nodeName === "DIV" && elem.classList.contains("server-info");
}

/**
 * @typedef ServerCardId
 * @type {string}
 */

/**
 * returns `null` if there is a collision
 * @param {ServerCardData} data
 * @returns {ServerCardId|null}
 */
function generateCardId(data) {
    const hopecard = `${data.name}@${data.addr}@${data.protver}`;
    if (hopecard in serverCardMap) {
        return null;
    }
    return hopecard;
}

/**
 * @param {ServerCardElement} card
 * @param {ServerCardId} id
 * @returns {void}
 */
function setCardId(card, id) {
    if (!isServerCard(card)) return;
    card.setAttribute("data-cardid", id);
}
/**
 * @param {ServerCardElement} card
 * @returns {ServerCardId}
 */
function getCardId(card) {
    if (!isServerCard(card)) return;
    return card.getAttribute("data-cardid");
}

/**
 * creates a new server card
 * @param {ServerCardData} data
 * @returns {ServerCardElement}
 */
function createServerCard(data) {
    const cardId = generateCardId(data);
    if (cardId === null) return;
    const SIC = createElementA("div", {"classList":["server-info"]});
    const BC = createElementA("span", {"classList":["slsi-bigchar"],"textContent":"C"});
    SIC.appendChild(BC);
    const ICONT = createElementA("div", {"classList":["slsi-icont"]});
    const ROW1 = createElementA("div", {"classList":["slsiic-row1"]});
    const NAME = createElementA("span", {"classList":["slsiic-name"],"textContent":data.name});
    ROW1.appendChild(NAME);
    const PCNT = createElementA("span", {"classList":["slsiic-pcnt", "S--unknown"],"textContent":"..."});
    ROW1.appendChild(PCNT);
    ICONT.appendChild(ROW1);
    const ROW2 = createElementA("div", {"classList":["slsiic-row2"]});
    const ADDR = createElementA("span", {"classList":["slsiic-addr"],"textContent":data.addr});
    ROW2.appendChild(ADDR);
    const VERS = createElementA("span", {"classList":["slsiic-vers"],"textContent":`Prtcl Ver: ${data.protver}`});
    ROW2.appendChild(VERS);
    const DATE = createElementA("span", {"classList":["slsiic-date"]});
    ROW2.appendChild(DATE);
    ICONT.appendChild(ROW2);
    SIC.appendChild(ICONT);
    setCardId(SIC, cardId);
    serverCardMap[cardId] = [data, SIC];
    SIC.addEventListener("click", ()=>{serverCardClick(cardId);});
    return SIC;
}

/**
 * @param {ServerCardElement} card
 * @param {number} curr current players
 * @param {number} max maximum players
 * @returns {void}
 */
function setServerCardPlayerCounts(card, curr, max) {
    if (!isServerCard(card)) return;
    /**@type {HTMLSpanElement} */
    const pcnt = card.children[1].children[0].children[1];
    pcnt.textContent = `${curr}/${max}`;
    pcnt.classList.remove("S--unknown");
    serverCardMap[getCardId(card)][0]["pcnt"] = [curr, max];
}

/**
 * @param {ServerCardElement} card
 * @param {string} time
 * @returns {void}
 */
function setServerCardLastPlayed(card, time) {
    if (!isServerCard(card)) return;
    /**@type {HTMLSpanElement} */
    const lpe = card.children[1].children[1].children[2];
    lpe.textContent = `Last Played: ${time} ago`;
    serverCardMap[getCardId(card)][0]["date"] = time;
}

/**
 * @param {ServerCardElement} card
 * @returns {void}
 */
function removeServerCard(card) {
    if (!isServerCard(card)) return;
    const id = getCardId(card);
    if (id in serverCardMap) {
        delete serverCardMap[id];
    }
    if (DOMServerList.contains(card)) {
        DOMServerList.removeChild(card);
    }
}

/**@type {ServerCardElement} */
let currActiveCard = null;

/**
 * @param {ServerCardElement} card
 * @returns {void}
 */
function setActiveCard(card) {
    if (card === null && currActiveCard === null) return;
    if (card !== null && !isServerCard(card)) return;
    if (currActiveCard !== null && card !== null && card.isSameNode(currActiveCard)) return;
    if (currActiveCard !== null) {
        currActiveCard.classList.remove("S--active");
    }
    if (card !== null) {
        card.classList.add("S--active");
    }
    currActiveCard = card;
}

/**
 * DO NOT CALL MANUALLY
 * @param {string} cardId
 * @returns {void}
 */
function serverCardClick(cardId) {
    console.log(cardId);
    /**@type {ServerCardElement} */
    const cardElem = serverCardMap[cardId][1];
    setActiveCard(cardElem);
}

// const dummyCard = createServerCard({"name":"DUMMY","addr":"0.0.0.0:8000","protver":"N/A"});
// setServerCardPlayerCounts(dummyCard, 0, 0);
// setServerCardLastPlayed(dummyCard, "1 year");
// DOMServerList.appendChild(dummyCard);
// const dummyCard2 = createServerCard({"name":"DUMMY","addr":"127.0.0.1:8000","protver":"N/A"});
// setServerCardPlayerCounts(dummyCard2, 0, 0);
// setServerCardLastPlayed(dummyCard2, "1 year");
// DOMServerList.appendChild(dummyCard2);

async function fetch_servers() {
    const m = JSON.parse(await invoke("fetch_servers", {}));
    for (const key in m) {
        // serverCardMap[key] = m[key];
        const cardD = m[key][0];
        const cardobj = createServerCard(cardD);
        // serverCardMap[key][1] = cardobj;
        DOMServerList.appendChild(cardobj);
        if ("date" in cardD) {
            setServerCardLastPlayed(cardobj, cardD["date"]);
        }
        if ("pcnt" in cardD) {
            setServerCardPlayerCounts(cardobj, cardD["pcnt"][0], cardD["pcnt"][1]);
        }
    }
}
async function store_servers() {
    await invoke("store_servers", {"servers":JSON.stringify(serverCardMap)});
}
fetch_servers();
