import {SkiaCanvas} from "./index.js"
import {NSkiaSurfaces} from "./index";

let canvas = new NSkiaSurfaces(1000, 1000);

const FRAME = 1;

let width = canvas.imWidth();
let height = canvas.imHeight();

let size = min(width, height)
let center = [size / 2, size / 2];

let chain_ring_radius = size / 2 * 100 / 100;
let triangle_radius = size / 2 * 53 / 100;

let rotation = frame;


chainRing(surface,)

function chainRing(surface: NSkiaSurfaces, center_x: number, center_y: number, radius: number, rotation: number, teethCount: number) {

    surface.cSave();
    surface.cTranslate(center_x,center_y);
    surface.cSave();
    canvas.cRotate(rotation);

    

}