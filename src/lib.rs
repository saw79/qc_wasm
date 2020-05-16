extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

mod core;
mod ecs;
mod util;
mod factory;
mod render;
mod movement;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[allow(unused_macros)]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen(raw_module = "../app.js")]
extern "C" {
    fn jsDrawImage(ctx: &CanvasRenderingContext2d, imgName: &str,
                   sx: u32, sy: u32, sw: u32, sh: u32,
                   x: f32, y: f32, w: f32, h: f32);
}

#[wasm_bindgen]
pub struct GameState {
    ctx: CanvasRenderingContext2d,
    width: u32,
    height: u32,
    tile_size: u32,
    player: ecs::Entity,
    enemies: Vec<ecs::Entity>,
}


#[wasm_bindgen]
impl GameState {
    #[wasm_bindgen(constructor)]
    pub fn new(ctx: CanvasRenderingContext2d, width: u32, height: u32) -> Self {
        console_log!("{}, {}", width, height);
        let tile_size: u32 = height / core::Y_TILES;
        GameState {
            ctx: ctx,
            width: width,
            height: height,
            tile_size: tile_size,
            player: factory::create_player(tile_size),
            enemies: vec![],
        }
    }

    // ------- public functions ---------------------

    pub fn tick(&mut self, dt_ms: f32) {
        self.update(dt_ms/1000.0);
        self.render();
    }

    pub fn add_mouse_click(&mut self, x: u32, y: u32) {
        let tx = x / self.tile_size;
        let ty = y / self.tile_size;
        self.player.target = Some(ecs::CTarget { x: tx, y: ty });
    }

    // ------- internal functions ---------------------

    fn update(&mut self, dt: f32) {
        movement::move_entity(&mut self.player, dt, self.tile_size);
    }

    fn render(&mut self) {
        render::draw_all(self);
    }
}

