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
        {name:"default",flavor:"default tileset",set:['-','!','+','@']},
        {name:"radicise",flavor:"Radicise's preferred tileset",set:['-','+','W','\u2588']},
        {name:"tsj845",flavor:"tsj845's peferred tileset",set:['-','*','&','#']},
        {name:"vihaan842",flavor:"vihaan842's peferred tileset",set:['\u3192','\u3193','\u3194','\u3195']}
    ]
});

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