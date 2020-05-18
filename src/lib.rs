extern crate wasm_bindgen;
extern crate web_sys;
extern crate pathfinding;

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
mod path_logic;
mod turn_logic;

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
    curr_turn: i32,
}


#[wasm_bindgen]
impl GameState {
    #[wasm_bindgen(constructor)]
    pub fn new(ctx: CanvasRenderingContext2d, width: u32, height: u32) -> Self {
        let mut camera = core::Camera::new(40, 40, width, height);

        let player = factory::create_player(5, 5);

        camera.x = 5.0;
        camera.y = 5.0;

        let enemies = vec![
            factory::create_target(2, 2),
            factory::create_target(4, 4),
            factory::create_target(6, 6),
            factory::create_target(8, 8),
            factory::create_target(10, 10),
            factory::create_target(12, 12),
            factory::create_target(14, 14),
        ];

        GameState {
            ctx: ctx,
            camera: camera,
            tile_grid: core::TileGrid::new(40, 40),
            player: player,
            enemies: enemies,
            curr_turn: -1,
        }
    }

    // ------- public functions ---------------------

    pub fn tick(&mut self, dt_ms: f32) {
        self.update(dt_ms/1000.0);
        self.render();
    }

    pub fn add_mouse_click(&mut self, mx: u32, my: u32) {
        let (wx, wy) = util::pixel_to_world(mx as f32, my as f32, &self.camera);
        self.process_new_target(wx as u32, wy as u32);
    }

    pub fn add_key_press(&mut self, code: u32) {
        console_log!("rust received code: {}", code);
    }

    // ------- internal functions ---------------------

    fn process_new_target(&mut self, wx: u32, wy: u32) -> Option<()> {
        if let Some(ref mut aq) = &mut self.player.action_queue {
            if aq.queue.len() > 0 {
                aq.queue.clear();
                return Some(());
            }
        }

        let x0 = self.player.logical_pos.as_ref()?.x;
        let y0 = self.player.logical_pos.as_ref()?.y;

        match path_logic::get_path(x0, y0, wx, wy, &self.tile_grid) {
            Some((mut path, cost)) => {
                path.remove(0);
                let new_q = path.into_iter().map(|p| ecs::Action::Move(p.0, p.1)).collect();
                self.player.action_queue.as_mut().map(|mut aq| aq.queue = new_q);
            },
            None => console_log!("  no path"),
        };

        Some(())
    }

    fn update(&mut self, dt: f32) -> Option<()> {
        turn_logic::compute_turns(self);

        movement::move_entity(&mut self.player, dt);
        self.camera.x = self.player.render_info.as_ref()?.x;
        self.camera.y = self.player.render_info.as_ref()?.y;
        Some(())
    }

    fn render(&mut self) {
        render::draw_all(self);
    }
}

