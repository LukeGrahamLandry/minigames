let Engine;
const sprites = new Image();
sprites.src = "assets/spritesheet.png";
sprites.onload = () => {
    console.log("Sprites loaded.");
}
const ctx = document.getElementById("game").getContext("2d");

const callbacks = {
    env: {
        console_log: function (ptr, len) {
            console.log(getWasmString(ptr, len));
        },
        random: () => Math.random(),
        performance_now: () => performance.now(),
        draw: function (textureId, x, y, alpha) {
            const size = 50;
            const outSize = 50;
            const cols = 3;
            const xS = textureId;// % cols;
            const yS = 0;//Math.round(textureId / cols);
            ctx.globalAlpha = alpha;
            ctx.drawImage(sprites, xS*size + (xS*2) + 1, yS*size + (yS*2) + 1, size, size, x, ctx.canvas.height - y, outSize, outSize);
            // ctx.strokeRect(x, ctx.canvas.height - y, outSize, outSize);
        }
    }
};

if (typeof WebAssembly === "object" && WebAssembly.instantiateStreaming !== undefined) {
    WebAssembly.instantiateStreaming(fetch("main.wasm"), callbacks).then(handleWasmLoaded);
} else {
    document.getElementById("loading").innerText = "Your browser doesn't support WASM.";
}

let level;
let ticker;

const targetMsPerFrame = 20;
function handleWasmLoaded(wasm) {
    Engine = wasm.instance.exports;
    console.log(Engine);
    document.getElementById("loading").style.display = "none";
    level = Engine.start_level();
    ticker = window.setInterval(tickFrame, targetMsPerFrame);

    // The game won't tick when the tab is unfocused so make sure we don't have a bunch of dt when it comes back.
    document.addEventListener("visibilitychange", () => {
        if (document.visibilityState === 'visible') {
            resumeGame();
        } else {
            pauseGame();
        }
    });

    document.addEventListener('keydown', (e) => Engine.update_key(level, e.key.charCodeAt(0), true));
    document.addEventListener('keyup', (e) => Engine.update_key(level, e.key.charCodeAt(0), false));

    initEditor();
}

function pauseGame() {
    console.log("Paused at " + new Date());
    window.clearInterval(ticker);
}

function resumeGame() {
    console.log("Resumed at " + new Date())
    Engine.reset_time(level);
    ticker = window.setInterval(tickFrame, targetMsPerFrame);
}

function tickFrame() {
    ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
    Engine.render(level);
}

function getWasmString(ptr, len) {
    const buffer = new Uint8Array(Engine.memory.buffer, ptr, len);
    return new TextDecoder().decode(buffer);
}
