import * as wasm from "wasm-game-of-life";

// wasm.greet("togo");
const pre = document.getElementById("game-of-life-canvas");
const universe = wasm.Universe.new();

const renderLoop = () => {
    pre.textContent = universe.render();
    universe.tick();

    requestAnimationFrame(renderLoop);
};

renderLoop();