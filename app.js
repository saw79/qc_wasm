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
  assets["health_bar"] = loadImage("assets/health_bar.png");
  assets["vision_wedge"] = loadImage("assets/enemy_vision_wedge.png");
  assets["btn_small_up"] = loadImage("assets/UIImages/button_small_up.png");
  assets["btn_small_down"] = loadImage("assets/UIImages/button_small_down.png");
  assets["btn_small_checked"] = loadImage("assets/UIImages/button_small_checked.png");
  assets["btn_skin_wait"] = loadImage("assets/UIImages/btn_wait_skin.png");
  assets["btn_skin_bag"] = loadImage("assets/UIImages/btn_bag_skin.png");
  assets["btn_skin_grab"] = loadImage("assets/UIImages/btn_grab_skin.png");
  assets["btn_skin_target"] = loadImage("assets/UIImages/btn_target_skin.png");
}

export function jsDrawImage(ctx, imgName, sx, sy, sw, sh, x, y, w, h) {
  ctx.drawImage(assets[imgName], sx, sy, sw, sh, x, y, w, h);
}

export function jsDrawImageAlpha(ctx, imgName, sx, sy, sw, sh, x, y, w, h, alpha) {
  ctx.globalAlpha = alpha;
  ctx.drawImage(assets[imgName], sx, sy, sw, sh, x, y, w, h);
  ctx.globalAlpha = 1;
}

export function jsAlphaToMain(ctx, ctx_alpha, alpha) {
  ctx.globalAlpha = alpha;
  ctx.drawImage(ctx_alpha.canvas, 0, 0);
  ctx.globalAlpha = 1;
}

export function jsDrawString(ctx, style, text, x, y) {
  if (style == "floating") {
    ctx.font = "32px Impact";
    ctx.fillStyle = "red";
    ctx.textAlign = "center";
  }
  else if (style == "patrol") {
    ctx.font = "32px Impact";
    ctx.fillStyle = "blue";
    ctx.textAlign = "center";
  }
  else if (style == "search") {
    ctx.font = "32px Impact";
    ctx.fillStyle = "yellow";
    ctx.textAlign = "center";
  }
  else if (style == "kill") {
    ctx.font = "32px Impact";
    ctx.fillStyle = "red";
    ctx.textAlign = "center";
  }
  else {
    console.log("jsDrawString (js) error unknown style!! " + style);
    return;
  }

  ctx.fillText(text, x, y);
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
  const canvas_off = document.createElement("canvas");
  canvas_off.width = width;
  canvas_off.height = height;
  const ctx_off = canvas_off.getContext("2d");

  // non-1 alplha buffer (fixes overlapping tile bug)
  const canvas_alpha = document.createElement("canvas");
  canvas_alpha.width = width;
  canvas_alpha.height = height;
  const ctx_alpha = canvas_alpha.getContext("2d");

  // load all assets
  loadAllImages();

  var state = new GameState(ctx_off, ctx_alpha, width, height);
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
  canvas.addEventListener("mousemove", function (e) {
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
  canvas.addEventListener("touchmove", function (e) {
    if (mouseDown) {
      handleDrag(e.changedTouches[0].clientX, e.changedTouches[0].clientY);
    }
  });

  window.addEventListener("keypress", function (e) {
    state.receive_key(e.keyCode);
  });

  // ----------- start loop ----------------

  var prevTime = 0;

  function mainLoop(timestamp) {
    state.tick(timestamp - prevTime);
    ctx.drawImage(canvas_off, 0, 0);

    prevTime = timestamp;
    requestAnimationFrame(mainLoop);
  }

  requestAnimationFrame(mainLoop);
}

run();

