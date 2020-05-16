import init, { GameState } from "./pkg/qc_wasm.js"

async function loadImage(path) {
  var img = new Image();
  img.src = path;
  return img;
}

async function loadImageData(path) {
  var img = new Image();
  img.src = path;
  await img.decode();
  const cv = document.createElement("canvas");
  const ctx = cv.getContext("2d");
  cv.width = img.width;
  cv.height = img.height;
  ctx.drawImage(img, 0, 0);
  return ctx.getImageData(0, 0, img.width, img.height).data;
}

function doDraw(ctx) {
  ctx.drawImage(img, 0, 0);
}

export function sayhi() {
  console.log("HI!");
} 

async function run() {
  await init();

  // parameters
  const width = 600;
  const height = 900;

  // setup
  const canvas = document.getElementById("canvas");
  canvas.width = width;
  canvas.height = height;
  const ctx = canvas.getContext("2d");

  // run game
  var state = new GameState(width, height);

  function mainLoop() {
    state.tick(ctx);

    requestAnimationFrame(mainLoop);
  }
  requestAnimationFrame(mainLoop);
}

run();

