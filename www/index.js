import { Cursive } from "retris";

const canvas = document.getElementById("cursive-wasm-canvas");
canvas.style.display = "block";
canvas.setAttribute("width", "1000");
canvas.setAttribute("height", "1000");
const ctx = canvas.getContext("2d");
ctx.fillStyle = "green";
// Add a rectangle at (10, 10) with size 100x100 pixels
ctx.fillRect(0, 0, 1000, 1000);
console.log("canvas is loaded", canvas !== null, ctx !== null);
Cursive.retris_with_canvas(canvas);
