import {SkiaCanvas } from "./index.js"
let canvas = new SkiaCanvas(1000,1000);
canvas.setLineWidth(10);
canvas.moveTo(0,0);
canvas.lineTo(50,50);
canvas.scale(3,4);
canvas.closePath();
canvas.bezierCurveTo(1,1000,100,100,4,105);
canvas.beginPath();
canvas.lineTo(100,100);
canvas.stroke();
canvas.beginPath();
canvas.save();
canvas.saveTo("./encoded.png");