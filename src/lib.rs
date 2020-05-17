extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[macro_use]
mod debug;
use debug::log;

mod core;
mod ecs;
mod util;
mod factory;
mod render;
mod movement;

#[wasm_bindgen(raw_module = "../app.js")]
extern "C" {
    fn jsDrawImage(ctx: &CanvasRenderingContext2d, imgName: &str,
                   sx: u32, sy: u32, sw: u32, sh: u32,
                   x: f32, y: f32, w: f32, h: f32);
}

#[wasm_bindgen]
pub struct GameState {
    ctx: CanvasRenderingContext2d,
    camera: core::Camera,
    tile_grid: core::TileGrid,
    player: ecs::Entity,
    enemies: Vec<ecs::Entity>,
}


#[wasm_bindgen]
impl GameState {
    #[wasm_bindgen(constructor)]
    pub fn new(ctx: CanvasRenderingContext2d, width: u32, height: u32) -> Self {
        GameState {
            ctx: ctx,
            camera: core::Camera::new(40, 40, width, height),
            tile_grid: core::TileGrid::new(40, 40),
            player: factory::create_player(5, 5),
            enemies: vec![],
        }
    }

    // ------- public functions ---------------------

    pub fn tick(&mut self, dt_ms: f32) {
        self.update(dt_ms/1000.0);
        self.render();
    }

    pub fn add_mouse_click(&mut self, mx: u32, my: u32) {
        let (wx, wy) = util::pixel_to_world(mx as f32, my as f32, &self.camera);
        self.player.target = Some(ecs::Target { x: wx as u32, y: wy as u32 });
    }

    pub fn add_key_press(&mut self, code: u32) {
        console_log!("rust received code: {}", code);
    }

    // ------- internal functions ---------------------

    fn update(&mut self, dt: f32) -> Option<()> {
        movement::move_entity(&mut self.player, dt);
        self.camera.x = self.player.visual_pos.as_ref()?.x;
        self.camera.y = self.player.visual_pos.as_ref()?.y;
        Some(())
    }

    fn render(&mut self) {
        render::draw_all(self);
    }
}

