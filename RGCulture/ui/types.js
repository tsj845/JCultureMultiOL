/**
 * @typedef Character
 * @type {String}
 */

/**
 * @typedef TileData
 * @type {[Character,Character,Character,Character]}
 */

/**
 * @typedef TileSet
 * @type {{set:TileData,name:String,flavor:String}}
 */

/**
 * @typedef TileSets
 * @type {{ofFile:boolean,origin:string,sets:TileSet[]}}
 */

/**
 * @typedef ModalElement
 * @type {HTMLDivElement}
 */

/**
 * @typedef BoardHook
 * @type {(x:number,y:number)=>void}
 */