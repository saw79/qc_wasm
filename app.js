import init, { GameState } from "./pkg/qc_wasm.js"

var assets = {};

function loadImage(path) {
  var img = new Image();
  img.src = path;
  return img;
}

function loadAllImages() {
  assets["prison_tiles"] = loadImage("assets/prison_tiles.png");
  assets["player_none"] = loadImage("assets/player_none.png");
  assets["prison_guard"] = loadImage("assets/prison_guard.png");
  assets["prison_soldier"] = loadImage("assets/prison_soldier.png");
  assets["prison_warden"] = loadImage("assets/prison_warden.png");
  assets["status_bg"] = loadImage("assets/status_bg.png");
  assets["status_cover"] = loadImage("assets/status_cover.png");
  assets["health_fill"] = loadImage("assets/health_fill.png");
  assets["cog_fill"] = loadImage("assets/cog_fill.png");
}

export function jsDrawImage(ctx, imgName, sx, sy, sw, sh, x, y, w, h, pixel_fix) {
  if (pixel_fix) {
    ctx.drawImage(assets[imgName], sx, sy, sw, sh, x-1, y-1, w+2, h+2);
  } else {
    ctx.drawImage(assets[imgName], sx, sy, sw, sh, x, y, w, h);
  }
}

async function run() {
  await init();

  // ----------- parameters ----------------

  const width = window.innerWidth;
  const height = window.innerHeight;

  // ----------- setup ----------------

  const canvas = document.getElementById("canvas");
  canvas.width = width;
  canvas.height = height;
  const ctx = canvas.getContext("2d");

  //canvas.webkitRequestFullScreen();

  // double buffering
  const canvas2 = document.createElement("canvas");
  canvas2.width = width;
  canvas2.height = height;
  const ctx2 = canvas2.getContext("2d");

  // load all assets
  loadAllImages();

  var state = new GameState(ctx2, width, height);
  var mouseDown = false;

  function handleClick(cx, cy, is_down) {
    let rect = canvas.getBoundingClientRect();
    let x = cx - rect.left;
    let y = cy - rect.top;
    state.receive_click(x, y, is_down);
  }

  function handleDrag(cx, cy) {
    let rect = canvas.getBoundingClientRect();
    let x = cx - rect.left;
    let y = cy - rect.top;
    state.receive_drag(x, y);
  }

  canvas.addEventListener("mousedown", function (e) {
    handleClick(e.clientX, e.clientY, true);
    mouseDown = true;
  });
  canvas.addEventListener("mouseup", function (e) {
    handleClick(e.clientX, e.clientY, false);
    mouseDown = false;
  });
  canvas.addEventListener("mousemove", function(e) {
    if (mouseDown) {
      handleDrag(e.clientX, e.clientY);
    }
  });
  canvas.addEventListener("touchstart", function (e) {
    handleClick(e.changedTouches[0].clientX, e.changedTouches[0].clientY, true);
    e.preventDefault();
    mouseDown = true;
  });
  canvas.addEventListener("touchend", function (e) {
    handleClick(e.changedTouches[0].clientX, e.changedTouches[0].clientY, false);
    e.preventDefault();
    mouseDown = false;
  });

  window.addEventListener("keypress", function(e) {
    state.receive_key(e.keyCode);
  });

  // ----------- start loop ----------------

  var prevTime = 0;

  function mainLoop(timestamp) {
    state.tick(timestamp - prevTime);
    ctx.drawImage(canvas2, 0, 0);

    prevTime = timestamp;
    requestAnimationFrame(mainLoop);
  }

  requestAnimationFrame(mainLoop);
}

run();

