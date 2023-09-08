import * as sim from "lib-simulation-wasm";

const simulation = new sim.Simulation();
const world = simulation.world();
console.log(world);


const viewport = document.getElementById('viewport');
const ctxt = viewport.getContext('2d');
const viewportWidth = viewport.width;
const viewportHeight = viewport.height;
const viewportScale = window.devicePixelRatio || 1;
// | This value determines how much physical pixels there are per
// | each single pixel on a canvas.
// |
// | Non-HiDPI displays usually have a pixel ratio of 1.0, which
// | means that drawing a single pixel on a canvas will lighten-up
// | exactly one physical pixel on the screen.
// |
// | if your display has a pixel ratio of 2.0, which means that for each
// | single pixel drawn on a canvas, there will be two physical
// | pixels modified by the browser.


viewport.width = viewportWidth * viewportScale;
viewport.height = viewportHeight * viewportScale;
// part 1: we're scaling-up canvas' *buffer*, so that it
// matches the screen's pixel ratio
viewport.style.width = viewportWidth + 'px';
viewport.style.height = viewportHeight + 'px';
// part 2: we're scaling-down canvas' *element*, because
// the browser will automatically multiply it by the pixel ratio in
// a moment.

ctxt.fillStyle = 'rgb(0, 0, 0)';


CanvasRenderingContext2D.prototype.drawTriangle =
    function (x, y, size, rotation) {
    this.beginPath();
    this.moveTo(
        x - Math.sin(rotation) * size,
        y + Math.cos(rotation) * size,
    );
    this.lineTo(
        x - Math.sin(rotation + 2.0 / 3.0 * Math.PI) * size * 1.5,
        y + Math.cos(rotation + 2.0 / 3.0 * Math.PI) * size * 1.5,
    );
    
    this.lineTo(
        x - Math.sin(rotation + 4.0 / 3.0 * Math.PI) * size * 1.5,
        y + Math.cos(rotation + 4.0 / 3.0 * Math.PI) * size * 1.5,
    );
    this.lineTo(
        x - Math.sin(rotation) * size,
        y + Math.cos(rotation) * size,
    );

    this.fillStyle = 'rgb(0, 0, 0)';
    this.fill();
};
CanvasRenderingContext2D.prototype.drawCircle =
    function(x, y, radius) {
        this.beginPath();
        this.arc(x, y, radius, 0, 2.0 * Math.PI);
        this.fillStyle = 'rgb(0, 0, 0)';
        this.fill();
    };

function redraw() {
    ctxt.clearRect(0, 0, viewportWidth, viewportHeight);
    
    const world = simulation.world();
    simulation.step();

    for (const animal of world.animals) {
        ctxt.drawTriangle(
            animal.x * viewportWidth,
            animal.y * viewportHeight,
            0.01 * viewportWidth,
            animal.rotation,
        );
    }

    for (const food of world.foods) {
        ctxt.drawCircle(
            food.x * viewportWidth,
            food.y * viewportHeight,
            (0.01 / 2.0) * viewportWidth,
        );
    }

    // requestAnimationFrame() schedules code only for the next frame.
    // Because we want for our simulation to continue forever, we've
    // gotta keep re-scheduling it:
    requestAnimationFrame(redraw);
}

redraw();