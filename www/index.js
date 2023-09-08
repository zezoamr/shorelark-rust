import * as sim from "lib-simulation-wasm";

let simulation = new sim.RouletteSimulation();
let selectiontype = "roulettewheel";

const messages = document.getElementById('messages');

function logMessage(message) {
    console.log(message);
    const messageElement = document.createElement('p');
    messageElement.textContent = message;
    messages.appendChild(messageElement);
    messages.scrollTop = messages.scrollHeight; // Scroll to the bottom of the messages div
}

document.getElementById('train').onclick = function() {
    logMessage(simulation.train());
};
document.getElementById('trainten').onclick = function() {
    for (let index = 0; index < 10; index++) {
        logMessage(simulation.train());
    }
};
document.getElementById('train100').onclick = function() {
    for (let index = 0; index < 100; index++) {
        logMessage(simulation.train());
    }
};

document.getElementById('reset').onclick = function() {
    logMessage("reset selection simulation");
    if(selectiontype === "roulettewheel") {
        simulation = new sim.RouletteSimulation();
        logMessage("roulettewheel selection simulation");
    } else if (selectiontype === "rank") {
        simulation = new sim.RankSimulation();
        logMessage("rank selection simulation");
    }
};

document.getElementById('roulettewheel').onclick = function() {
    simulation = new sim.RouletteSimulation();
    selectiontype = "roulettewheel";
    logMessage("roulettewheel selection simulation");
};

document.getElementById('rank').onclick = function() {
    simulation = new sim.RankSimulation();
    selectiontype = "rank";
    logMessage("rank selection simulation");
};

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
    const margin = size * 2;
    // Check if the position is within the bounds of the canvas
    if (x - margin < 0 || x + margin > viewportWidth || y - margin < 0 || y + margin > viewportHeight) {
        return; // Skip drawing this shape
    }
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

    this.fillStyle = 'rgb(255, 255, 255)'; // A nice white color
    this.fill();
};
CanvasRenderingContext2D.prototype.drawCircle =
    function(x, y, radius) {
        const margin = radius * 2;
        // Check if the position is within the bounds of the canvas
        if (x - margin < 0 || x + margin > viewportWidth || y - margin < 0 || y + margin > viewportHeight) {
            return; // Skip drawing this shape
        }
        this.beginPath();
        this.arc(x, y, radius, 0, 2.0 * Math.PI);
        this.fillStyle = 'rgb(0, 255, 128)'; // A nice green color
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


const logoElement = document.getElementById('logo');

    // Define the logo text
    const logoText = 
`     _____ _                    _            _    
    / ____| |                  | |          | |   
   | (___ | |__   ___  _ __ ___| | __ _ _ __| | __
    \\___ \\| '_ \\ / _ \\| '__/ _ \\ |/ _\` | '__| |/ /
    ____) | | | | (_) | | |  __/ | (_| | |  |   < 
   |_____/|_| |_|\\___/|_|  \\___|_|\\__,_|_|  |_|\\_\\`;

    // Set the text content of the logo div element to the logo text
    logoElement.textContent = logoText;