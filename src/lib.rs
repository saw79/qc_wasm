extern crate wasm_bindgen;
extern crate web_sys;
extern crate pathfinding;
extern crate rand;

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
mod render_ui;
mod movement;
mod path_logic;
mod turn_logic;
mod animation_logic;
mod ai_logic;

#[wasm_bindgen(raw_module = "../app.js")]
extern "C" {
    fn jsDrawImage(ctx: &CanvasRenderingContext2d, imgName: &str,
                   sx: u32, sy: u32, sw: u32, sh: u32,
                   x: f32, y: f32, w: f32, h: f32,
                   pixel_fix: bool);
}

#[wasm_bindgen]
pub struct GameState {
    ctx: CanvasRenderingContext2d,
    camera: core::Camera,
    tile_grid: core::TileGrid,
    entities: Vec<ecs::Entity>,
    curr_turn: usize,
    last_click_pos: (u32, u32),
    last_camera_pos: (f32, f32),
    paused: bool,
}


#[wasm_bindgen]
impl GameState {
    #[wasm_bindgen(constructor)]
    pub fn new(ctx: CanvasRenderingContext2d, width: u32, height: u32) -> Self {
        let mut camera = core::Camera::new(40, 40, width, height);

        let (px, py) = (5, 5);

        camera.x = px as f32;
        camera.y = py as f32;

        let entities = vec![
            factory::create_player(px, py),
            factory::create_enemy(10, 10, "prison_guard"),
        ];

        GameState {
            ctx: ctx,
            camera: camera,
            tile_grid: core::TileGrid::new(40, 40),
            entities: entities,
            curr_turn: 0,
            last_click_pos: (0, 0),
            last_camera_pos: (0.0, 0.0),
            paused: false,
        }
    }

    // ------- public functions ---------------------

    pub fn tick(&mut self, dt_ms: f32) {
        self.update(dt_ms/1000.0);
        self.render();
    }

    pub fn receive_click(&mut self, mx: u32, my: u32, is_down: bool) {
        if is_down {
            self.last_click_pos = (mx, my);
            self.last_camera_pos = (self.camera.x, self.camera.y);
        } else {
            let (mx0, my0) = self.last_click_pos;
            let dx = (mx as i32 - mx0 as i32).abs();
            let dy = (my as i32 - my0 as i32).abs();
            if dx + dy < 10 {
                let (wx, wy) = util::pixel_to_world(mx as f32, my as f32, &self.camera);
                self.process_click(wx as u32, wy as u32);
            }
        }
    }

    pub fn receive_drag(&mut self, mx: u32, my: u32) {
        let (mx0, my0) = self.last_click_pos;
        let (cx0, cy0) = self.last_camera_pos;
        let dx: f32 = (mx0 as f32 - mx as f32) / self.camera.tile_pix as f32;
        let dy: f32 = (my0 as f32 - my as f32) / self.camera.tile_pix as f32;
        self.camera.x = cx0 + dx;
        self.camera.y = cy0 + dy;
    }

    pub fn receive_key(&mut self, code: u32) {
        console_log!("rust received code: {}", code);
    }

    // ------- internal functions ---------------------

    fn process_click(&mut self, wx: u32, wy: u32) -> Option<()> {
        // if has actions, this click means ABORT
        if let Some(ref mut aq) = &mut self.entities[0].action_queue {
            if aq.queue.len() > 0 {
                aq.queue.clear();
                return Some(());
            }
        }

        // current position needed for everything
        let x0 = self.entities[0].logical_pos.as_ref()?.x;
        let y0 = self.entities[0].logical_pos.as_ref()?.y;

        if x0 == wx && y0 == wy {
            // self click
            self.entities[0].action_queue.as_mut()
                .map(|aq| aq.queue.push(ecs::Action::Wait));
        }
        else {
            // check path, either move or attack enemy
            match path_logic::get_path(x0, y0, wx, wy, &self.tile_grid) {
                Some((mut path, _cost)) => {
                    path.remove(0);
                    let new_q = path.into_iter().map(|p| ecs::Action::Move(p.0, p.1)).collect();
                    self.entities[0].action_queue.as_mut().map(|aq| aq.queue = new_q);
                },
                None => console_log!("  no path"),
            };
        }

        Some(())
    }

    fn update(&mut self, dt: f32) {
        turn_logic::compute_turns(self);

        movement::move_entities(self, dt);

        animation_logic::compute_animations(self, dt);
    }

    fn render(&mut self) {
        render::draw_all(self);
        render_ui::draw_ui(self);
    }
}

