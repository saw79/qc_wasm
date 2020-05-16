import init, { GameState } from "./pkg/qc_wasm.js"

var assets = {};

function loadImage(path) {
  var img = new Image();
  img.src = path;
  return img;
}

function loadAllImages() {
  assets["player_none"] = loadImage("assets/player_none.png");
  assets["prison_soldier"] = loadImage("assets/prison_soldier.png");
  assets["prison_tiles"] = loadImage("assets/prison_tiles.png");
  assets["target"] = loadImage("assets/target.png");
  assets["prison_guard"] = loadImage("assets/prison_guard.png");
  assets["prison_stairs"] = loadImage("assets/prison_stairs.png");
  assets["prison_warden"] = loadImage("assets/prison_warden.png");
  assets["x."] = loadImage("assets/x.png");
}

export function jsDrawImage(ctx, imgName, sx, sy, sw, sh, x, y, w, h) {
  ctx.drawImage(assets[imgName], sx, sy, sw, sh, x-1, y-1, w+2, h+2);
}

async function run() {
  await init();

  // ----------- parameters ----------------

  const width = window.innerWidth;
  const height = window.innerHeight;
  console.log("setting width: " + width)
  console.log("setting height: " + height)

  // ----------- setup ----------------

  const canvas = document.getElementById("canvas");
  canvas.width = width;
  canvas.height = height;
  const ctx = canvas.getContext("2d");

  canvas.webkitRequestFullScreen();

  // double buffering
  const canvas2 = document.createElement("canvas");
  canvas2.width = width;
  canvas2.height = height;
  const ctx2 = canvas2.getContext("2d");

  // load all assets
  loadAllImages();

  var state = new GameState(ctx2, width, height);

  canvas.addEventListener("mousedown", function(e) {
    let rect = canvas.getBoundingClientRect();
    let x = e.clientX - rect.left;
    let y = e.clientY - rect.top;
    state.add_mouse_click(x, y);
  });

  // ----------- start loop ----------------

  var prevTime = new Date().getTime();

  function mainLoop(timestamp) {
    state.tick(timestamp - prevTime);
    ctx.drawImage(canvas2, 0, 0);

    prevTime = timestamp;
    requestAnimationFrame(mainLoop);
  }

  requestAnimationFrame(mainLoop);
}

run();

