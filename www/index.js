import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";
import { Universe, Cell } from "wasm-game-of-life";
import shapes from "./shapes"

const CELL_SIZE = 7; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const universe = Universe.new(128, 48);
const width = universe.width();
const height = universe.height();

const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;
canvas.addEventListener("click", event => {
    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;

    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

    universe.toggle_cell(row, col);

    drawGrid();
    drawCells();
});

// TODO canvas.addEventListener("mousemove", event => { ... })
//  mousedown, mouseup, mousemove

console.log(shapes)

let ticksPerFrame = 30;
const ticksPerFrameElem = document.getElementById("ticks-per-frame");
ticksPerFrameElem.setAttribute('min', 1)
ticksPerFrameElem.setAttribute('max', 100)
ticksPerFrameElem.setAttribute('value', ticksPerFrame)
ticksPerFrameElem.addEventListener("click", event => {
    ticksPerFrame = event.target.valueAsNumber
    ticksPerFrameElem.setAttribute('value', ticksPerFrame)
    ticksPerFrameLabelElem.setAttribute("textContent", ticksPerFrame)
});

const ticksPerFrameLabelElem = document.getElementById("ticks-per-frame-label");

const ctx = canvas.getContext('2d');

let animationId = null;

const isPaused = () => {
    return animationId === null;
};

const playPauseButton = document.getElementById("play-pause");

const play = () => {
    playPauseButton.textContent = "⏸";
    renderLoop();
};

const pause = () => {
    playPauseButton.textContent = "▶️";
    cancelAnimationFrame(animationId);
    animationId = null;
};

playPauseButton.addEventListener("click", event => {
    if (isPaused()) {
        play();
    } else {
        pause();
    }
});

const tickNumElement = document.getElementById("tick-num");
const frameNumElement = document.getElementById("frame-num");

let tickId = 0;
let frameId = 0;
const renderLoop = () => {
    tickId++;
    tickNumElement.textContent = `tick: ${tickId}`
    frameNumElement.textContent = `generation: ${frameId}`
    ticksPerFrameElem.setAttribute('value', ticksPerFrame)
    ticksPerFrameLabelElem.setAttribute("textContent", ticksPerFrame)

    if (tickId % ticksPerFrame === 0) {
        frameId++;
        universe.tick();
        drawGrid();
        drawCells();
    }

    animationId = requestAnimationFrame(renderLoop);
};

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
};

const getIndex = (row, column) => {
    return row * width + column;
};

const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);

            ctx.fillStyle = cells[idx] === Cell.Dead
                ? DEAD_COLOR
                : ALIVE_COLOR;

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
};

drawGrid();
drawCells();
play();