/**
 * @typedef Character
 * @type {string}
 */

/**
 * @typedef AssetLocation
 * @type {string}
 */

/**
 * @typedef TileData
 * @type {[Character,Character,Character,Character]}
 */

/**
 * @typedef TileSet
 * @type {{isText:true,set:TileData,name:string,flavor:string}|{isText:false,set:AssetLocation,name:string,flavor:string,ext:string}}
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

/**
 * @typedef ServerCardData
 * @type {{name:string,addr:string,protver:string}}
 */

/**
 * @typedef ServerCardElement
 * @type {HTMLDivElement}
 */
