extern crate wasm_bindgen;
extern crate web_sys;
extern crate pathfinding;
extern crate rand;

use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[macro_use]
mod debug;
use debug::log;

mod constants;
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
mod combat_logic;
mod bresenham;

#[wasm_bindgen(raw_module = "../app.js")]
extern "C" {
    fn jsDrawImage(ctx: &CanvasRenderingContext2d, imgName: &str,
                   sx: u32, sy: u32, sw: u32, sh: u32,
                   x: u32, y: u32, w: u32, h: u32);
    fn jsDrawImageAlpha(ctx: &CanvasRenderingContext2d, imgName: &str,
                        sx: u32, sy: u32, sw: u32, sh: u32,
                        x: u32, y: u32, w: u32, h: u32,
                        alpha: f32);
    fn jsDrawString(ctx: &CanvasRenderingContext2d, text: &str, x: u32, y: u32);
}

#[wasm_bindgen]
pub struct GameState {
    ctx: CanvasRenderingContext2d,
    camera: core::Camera,
    tile_grid: core::TileGrid,
    entity_map: HashMap<usize, ecs::Entity>,
    curr_turn: usize,
    last_click_pos: (u32, u32),
    last_camera_pos: (f32, f32),
    paused: bool,
    floating_texts: Vec<core::FloatingText>,
}


#[wasm_bindgen]
impl GameState {
    #[wasm_bindgen(constructor)]
    pub fn new(ctx: CanvasRenderingContext2d, width: u32, height: u32) -> Self {
        let mut camera = core::Camera::new(40, 40, width, height);

        let (px, py) = (5, 5);

        camera.x = px as f32;
        camera.y = py as f32;

        let mut tile_grid = core::TileGrid::new(40, 40);
        tile_grid.update_visibility(px, py);

        let mut entity_map = HashMap::new();
        entity_map.insert(0, factory::create_player(px, py));
        entity_map.insert(1, factory::create_enemy(10, 10, "prison_guard"));

        GameState {
            ctx: ctx,
            camera: camera,
            tile_grid: tile_grid,
            entity_map: entity_map,
            curr_turn: 0,
            last_click_pos: (0, 0),
            last_camera_pos: (0.0, 0.0),
            paused: false,
            floating_texts: vec![],
        }
    }

    // ------- public functions ---------------------

    pub fn tick(&mut self, dt_ms: f32) {
        if !self.paused {
            self.update(dt_ms/1000.0);
        }

        self.render();
    }

    pub fn receive_click(&mut self, mx: u32, my: u32, is_down: bool) {
        if is_down {
            self.last_click_pos = (mx, my);
            self.last_camera_pos = (self.camera.x, self.camera.y);
            self.paused = true;
        } else {
            let (mx0, my0) = self.last_click_pos;
            let dx = (mx as i32 - mx0 as i32).abs();
            let dy = (my as i32 - my0 as i32).abs();
            if dx + dy < 20 {
                let (wx, wy) = util::pixel_to_world(mx, my, &self.camera);
                self.process_click(wx, wy);
            }
            self.paused = false;
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

    fn process_click(&mut self, wx: f32, wy: f32) -> Option<()> {
        // if has actions, this click means ABORT
        if let Some(aq) = &mut self.entity_map.get_mut(&0)?.action_queue {
            if aq.queue.len() > 0 {
                aq.queue.clear();
                return Some(());
            }
        }

        // current position needed for everything
        let x0 = self.entity_map.get(&0)?.logical_pos.as_ref()?.x;
        let y0 = self.entity_map.get(&0)?.logical_pos.as_ref()?.y;

        let wx_int = wx as i32;
        let wy_int = wy as i32;

        if x0 == wx_int && y0 == wy_int {
            // self click
            self.process_self_click();
        }
        else {
            let mut clicked_enemy = false;
            if let Some(id) = self.get_entity_at(wx_int, wy_int) {
                self.entity_map.get_mut(&0)?.entity_target = Some(ecs::EntityTarget { id: id });
                clicked_enemy = true;
            }

            // check path, either move or attack enemy
            match path_logic::get_path(x0, y0, wx_int, wy_int, &self.tile_grid) {
                Some((mut path, _cost)) => {
                    path.remove(0);
                    if path.len() == 0 {
                        console_log!("rust lib: path len is 0, how???");
                    } else {
                        if clicked_enemy {
                            self.entity_map.get_mut(&0)?.action_queue.as_mut()?.queue =
                                vec![ecs::Action::Move(path[0].0, path[0].1)];
                        } else {
                            let new_q = path.into_iter()
                                .map(|p| ecs::Action::Move(p.0, p.1)).collect();
                            self.entity_map.get_mut(&0)?.action_queue.as_mut()
                                .map(|aq| aq.queue = new_q);
                        }
                    }
                },
                None => console_log!("  no path"),
            };
        }

        Some(())
    }

    fn process_self_click(&mut self) -> Option<()> {
        self.entity_map.get_mut(&0)?.action_queue.as_mut()
            .map(|aq| aq.queue.push(ecs::Action::Wait));

        Some(())
    }

    fn update(&mut self, dt: f32) {
        turn_logic::compute_turns(self);
        combat_logic::process_combat(self);
        movement::move_entities(self, dt);
        movement::move_floating_texts(self, dt);
        animation_logic::compute_animations(self, dt);
    }

    fn render(&mut self) {
        render::draw_all(self);
        render_ui::draw_ui(self);
    }

    fn get_entity_at(&self, x: i32, y: i32) -> Option<usize> {
        for (&id, entity) in &self.entity_map {
            if id == 0 { continue; }

            if let Some(pos) = &entity.logical_pos {
                if pos.x == x && pos.y == y {
                    return Some(id);
                }
            }
        }

        None
    }
}

