const normal_colors = frozen({
    "-1":"#808080",
    0:"#d8070b",
    1:"#0060fb",
    2:"#00af24",
    3:"#ffaf2a",
    4:"#c309fa",
    5:"#00affc"
});

const volatile_colors = frozen({
    "-1":"#b2b2b2",
    0:"#ff0a0f",
    1:"#5e87fb",
    2:"#00ff39",
    3:"#ffff3d",
    4:"#ff0efb",
    5:"#00ffff"
});

/**@type {Readonly<TileSets>} */
const builtinSets = frozen({
    ofFile:false,
    origin:"defaults",
    sets:[
        {name:"default",flavor:"default tileset",set:['-','!','+','@'],isText:true},
        {name:"radicise",flavor:"Radicise's preferred tileset",set:['-','+','W','\u2588'],isText:true},
        {name:"tsj845",flavor:"tsj845's peferred tileset",set:['-','*','&','#'],isText:true},
        {name:"vihaan842",flavor:"vihaan842's peferred tileset",set:['\u3192','\u3193','\u3194','\u3195'],isText:true},
        {name:"image test",flavor:"TEST TILESET MAKING USE OF IMAGES",isText:false,set:"./assets/tiles/test/",ext:".svg"}
    ]
});

/**@type {Map<string,[Document,Document,Document,Document]>} */
let imageTilesCache = {};

/**@type {TileSets[]} */
const atilesets = [builtinSets];

let current_set = [0, 0];
let csetraw = atilesets[current_set[0]].sets[current_set[1]].set;

/**
 * the maximum board size
 * @type {number}
 * @constant
 * @global
 */
const MAX_SIZE = 2600;

let SVGLoadDispatch = 0;

/**
 * 
 * @param {TileSet} tset
 * @param {false} tset.isText
 * @returns {Promise<void>}
 */
function loadSvgTiles(tset) {
    // let doc = new DOMParser().parseFromString(await invoke("fetch_content", tset.set+"1.svg"), "image/svg+xml");
    return new Promise((r,_)=>{
        let svgs = [];
        let c = 4;
        for (let i = 1; i < 5; i ++) {
            invoke("fetch_content", {"path":`${tset.set}${i}.svg`}).then((v)=>{
                svgs[i-1] = new DOMParser().parseFromString(v, "image/svg+xml");
                c --;
                if (c === 0) {
                    imageTilesCache[`${tset.set}${tset.ext}`] = svgs;
                    r();
                }
            });
        }
    });
}

async function loadAllSvgs() {
    await new Promise((r,_)=>{
        let c = 0;
        for (const sets of atilesets) {
            for (const set of sets.sets) {
                if (!set.isText) {
                    c ++;
                    loadSvgTiles(set).then(() => {
                        c --;
                        if (c === 0) {
                            r();
                        }
                    });
                }
            }
        }
    });
}