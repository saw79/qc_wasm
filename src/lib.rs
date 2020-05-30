extern crate wasm_bindgen;
extern crate web_sys;
extern crate pathfinding;
extern crate rand;

use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, window};

#[macro_use]
mod debug;
use debug::log;

mod constants;
mod core;
mod tile_grid;
mod ecs;
mod turn_queue;
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
mod level_gen;
mod user_interface;

const TIMING: bool = false;

fn time() -> f64 {
    window().unwrap().performance().unwrap().now()
}

#[wasm_bindgen(raw_module = "../app.js")]
extern "C" {
    fn jsDrawImage(ctx: &CanvasRenderingContext2d, imgName: &str,
                   sx: i32, sy: i32, sw: i32, sh: i32,
                   x: i32, y: i32, w: i32, h: i32);
    fn jsDrawImageAlpha(ctx: &CanvasRenderingContext2d, imgName: &str,
                        sx: i32, sy: i32, sw: i32, sh: i32,
                        x: i32, y: i32, w: i32, h: i32,
                        alpha: f32);
    fn jsAlphaToMain(ctx: &CanvasRenderingContext2d, ctx_alpha: &CanvasRenderingContext2d,
                     alpha: f32);
    fn jsDrawString(ctx: &CanvasRenderingContext2d, style: &str, text: &str, x: i32, y: i32);
}

#[wasm_bindgen]
pub struct GameState {
    t0: f64,
    ctx: CanvasRenderingContext2d,
    ctx_alpha: CanvasRenderingContext2d,
    ui: user_interface::UserInterface,
    camera: core::Camera,
    tile_grid: tile_grid::TileGrid,
    entity_map: HashMap<ecs::EntityId, ecs::Entity>,
    turn_queue: turn_queue::TurnQueue,
    last_click_pos: (i32, i32),
    last_camera_pos: (f32, f32),
    paused: bool,
    floating_texts: Vec<core::FloatingText>,
    enemy_visible_prev: bool,
}


#[wasm_bindgen]
impl GameState {
    #[wasm_bindgen(constructor)]
    pub fn new(ctx: CanvasRenderingContext2d, ctx_aplha: CanvasRenderingContext2d,
               width: i32, height: i32) -> Self {
        // initialize camera/grid/level
        let mut camera = core::Camera::new(40, 40, width, height);

        // initialize user interface
        let ui = user_interface::UserInterface::new(camera.canvas_width,
                                                    camera.canvas_height,
                                                    camera.tile_pix);

        let mut tile_grid = tile_grid::TileGrid::new(40, 40);

        let (px, py) = tile_grid.get_random_floor();

        camera.x = px as f32;
        camera.y = py as f32;
        
        // zoom out test
        /*let zoom: f32 = 3.0;
        camera.width *= zoom;
        camera.height *= zoom;
        camera.tile_pix = (camera.tile_pix as f32 / zoom) as i32;*/

        // ----- add entities ------

        let mut entity_map = HashMap::new();
        let mut turn_queue = turn_queue::TurnQueue::new();

        let id = util::get_next_id(&entity_map);
        entity_map.insert(id, factory::create_player(px, py));
        turn_queue.add(id);

        let radius = entity_map.get(&0).unwrap().vision_info.as_ref().unwrap().radius;
        tile_grid.update_visibility(px, py, radius);

        let num_enemies = 2;//20;
        let num_orbs = 1;//6;

        for _ in 0..num_enemies {
            let (ex, ey) = tile_grid.get_random_floor();
            let id = util::get_next_id(&entity_map);
            entity_map.insert(id, factory::create_enemy(ex, ey, "prison_guard"));
            turn_queue.add(id);
        }

        for _ in 0..num_orbs {
            let (ox, oy) = tile_grid.get_random_floor();
            let id = util::get_next_id(&entity_map);
            entity_map.insert(id, factory::create_orb(ox, oy, "health_orb"));

            let (ox, oy) = tile_grid.get_random_floor();
            let id = util::get_next_id(&entity_map);
            entity_map.insert(id, factory::create_orb(ox, oy, "cognition_orb"));
        }

        for _ in 0..2 {
            let (ox, oy) = tile_grid.get_random_floor();
            let id = util::get_next_id(&entity_map);
            entity_map.insert(id, factory::create_orb(ox, oy, "rejuvination_orb"));
        }

        GameState {
            t0: 0.0,
            ctx: ctx,
            ctx_alpha: ctx_aplha,
            ui: ui,
            camera: camera,
            tile_grid: tile_grid,
            entity_map: entity_map,
            turn_queue: turn_queue,
            last_click_pos: (0, 0),
            last_camera_pos: (0.0, 0.0),
            paused: false,
            floating_texts: vec![],
            enemy_visible_prev: false,
        }
    }

    // ------- public functions ---------------------

    pub fn tick(&mut self, dt_ms: f32) {
        if TIMING { console_log!("dead: {}", time()-self.t0); }

        if TIMING { self.t0 = time(); }
        if !self.paused {
            self.update(dt_ms/1000.0);
        }
        if TIMING { console_log!("update: {}", time()-self.t0); }

        if TIMING { self.t0 = time(); }
        self.render();
        if TIMING { console_log!("render: {}", time()-self.t0); }

        if TIMING { self.t0 = time(); }

        //console_log!("# entities: {}", self.entity_map.len());
    }

    pub fn receive_click(&mut self, mx: i32, my: i32, is_down: bool) {
        // USER INTERFACE
        if is_down {
            match self.ui.log_click_down(mx, my) {
                Some(_bt) => {
                    return;
                },
                None => {},
            };
        } else {
            match self.ui.log_click_up(mx, my) {
                Some(bt) => {
                    match bt {
                        user_interface::ButtonType::WAIT   => {self.player_wait();},
                        user_interface::ButtonType::BAG    => console_log!("bag not implemented"),
                        user_interface::ButtonType::GRAB   => {self.player_grab();},
                        user_interface::ButtonType::TARGET => console_log!("target not implemented"),
                        user_interface::ButtonType::ATTACK => {self.player_attack();},
                    };
                    return;
                },
                None => {},
            };
        }

        // GAME PLAY
        if is_down {
            self.last_click_pos = (mx, my);
            self.last_camera_pos = (self.camera.x, self.camera.y);
            self.paused = true;
        } else {
            let (mx0, my0) = self.last_click_pos;
            let dx = (mx - mx0).abs();
            let dy = (my - my0).abs();
            if dx + dy < 20 {
                let (wx, wy) = util::pixel_to_world(mx, my, &self.camera);
                self.process_click(wx, wy);
            }
            self.paused = false;
        }
    }

    pub fn receive_drag(&mut self, mx: i32, my: i32) {
        if let Some(_) = self.ui.button_down {
            return;
        }

        let (mx0, my0) = self.last_click_pos;
        let (cx0, cy0) = self.last_camera_pos;
        let dx: f32 = (mx0 as f32 - mx as f32) / self.camera.tile_pix as f32;
        let dy: f32 = (my0 as f32 - my as f32) / self.camera.tile_pix as f32;
        self.camera.x = cx0 + dx;
        self.camera.y = cy0 + dy;
    }

    pub fn receive_key(&mut self, code: i32) {
        match code {
            constants::KEY_A => self.player_attack(),
            constants::KEY_B => { console_log!("OPEN BAG"); Some(()) },
            constants::KEY_G => self.player_grab(),
            constants::KEY_T => { console_log!("TARGET"); Some(()) },
            constants::KEY_W => self.player_wait(),
            _ => { console_log!("received key {}", code); Some(()) },
        };
    }

    // ------- internal functions ---------------------

    fn update(&mut self, dt: f32) {
        turn_logic::process_turns(self);
        combat_logic::process_combat(self);
        movement::move_entities(self, dt);
        movement::move_floating_texts(self, dt);
        animation_logic::compute_animations(self, dt);

        let enemy_visible = self.is_enemy_visible();
        if (enemy_visible && !self.enemy_visible_prev) || self.is_player_visible() {
            if let Some(pl) = self.entity_map.get_mut(&0) {
                if let Some(aq) = pl.action_queue.as_mut() {
                    aq.queue.clear();
                }
            }
        }

        self.enemy_visible_prev = enemy_visible;

        self.process_deaths();
    }

    fn process_deaths(&mut self) {
        for (id, entity) in self.entity_map.iter() {
            if entity.dead {
                self.turn_queue.remove(*id);
            }
        }

        self.entity_map.retain(|_, e| !e.dead);
    }

    pub fn update_visibility(&mut self) {
        if let Some(player) = self.entity_map.get(&0) {
            if let Some(lp) = player.logical_pos.as_ref() {
                if let Some(vi) = player.vision_info.as_ref() {
                    self.tile_grid.update_visibility(lp.x, lp.y, vi.radius);
                }
            }
        }
    }

    fn render(&mut self) {
        render::draw_all(self);
        render_ui::draw_ui(self);
    }

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

        // now only accept clicks in areas that have been seen
        if self.tile_grid.get_visibility(wx_int, wy_int) == &tile_grid::Visibility::UNSEEN {
            return Some(());
        }

        if x0 == wx_int && y0 == wy_int {
            // self click
            self.process_self_click();
        }
        else {
            let mut clicked_enemy = false;
            if let Some(id) = self.get_entity_at(wx_int, wy_int) {
                if let Some(_) = self.entity_map.get(&id)?.combat_info.as_ref() {
                    self.entity_map.get_mut(&0)?.entity_target = Some(ecs::EntityTarget { id: id });
                    clicked_enemy = true;
                }
            }

            // check path to click pos, either move or attack enemy
            match path_logic::get_path(x0, y0, wx_int, wy_int, &self.tile_grid) {
                Some((mut path, _cost)) => {
                    path.remove(0);
                    if path.len() == 0 {
                        console_log!("rust lib: path len is 0 but returned some, how???");
                    } else if !clicked_enemy {
                        let new_q = path.into_iter()
                            .map(|p| ecs::Action::Move(p.0, p.1)).collect();
                        self.entity_map.get_mut(&0)?.action_queue.as_mut()
                            .map(|aq| aq.queue = new_q);
                    }
                },
                None => console_log!("  no path"),
            };
        }

        Some(())
    }

    fn process_self_click(&mut self) {
        self.player_wait();
    }

    fn player_wait(&mut self) -> Option<()> {
        self.entity_map.get_mut(&0)?.action_queue.as_mut()
            .map(|aq| aq.queue.push(ecs::Action::Wait));

        Some(())
    }

    fn try_attack_tile(&mut self, x: i32, y: i32) -> Option<bool> {
        match self.get_entity_at(x, y) {
            Some(id) => {
                self.entity_map.get_mut(&0)?.entity_target = Some(ecs::EntityTarget { id: id });
                Some(true)
            },
            None => Some(false),
        }
    }

    fn player_attack(&mut self) -> Option<()> {
        let lp = self.entity_map.get(&0)?.logical_pos.as_ref()?;
        let (x0, y0) = (lp.x, lp.y);

        for (dx, dy) in &[(1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1)] {
            match self.try_attack_tile(x0 + dx, y0 + dy)? {
                true => return Some(()),
                false => {},
            }
        }

        None
    }

    fn get_entity_at(&self, x: i32, y: i32) -> Option<ecs::EntityId> {
        for (id, entity) in self.entity_map.iter() {
            if *id == 0 { continue; }

            if let Some(pos) = &entity.logical_pos {
                if pos.x == x && pos.y == y {
                    return Some(*id);
                }
            }
        }

        None
    }

    fn is_enemy_visible(&self) -> bool {
        for (id, entity) in self.entity_map.iter() {
            if *id > 0 {
                if let Some(lp) = entity.logical_pos.as_ref() {
                    if self.tile_grid.get_visibility(lp.x, lp.y) == &tile_grid::Visibility::VISIBLE {
                        if let Some(_) = entity.combat_info.as_ref() {
                            return true;
                        }
                    }
                }
            }
        }

        return false;
    }

    fn is_player_visible(&self) -> bool {
        for (id, entity) in self.entity_map.iter() {
            if *id > 0 {
                if let Some(vi) = entity.vision_info.as_ref() {
                    if vi.alert_state == ecs::AlertState::KILL {
                        return true;
                    }
                }
            }
        }

        return false;
    }

    fn player_grab(&mut self) -> Option<()> {
        let mut player_actions = vec![];

        let (px, py) = {
            let lp = self.entity_map.get(&0)?.logical_pos.as_ref()?;
            (lp.x, lp.y)
        };

        for (id, entity) in self.entity_map.iter_mut() {
            if *id > 0 {
                grab_entity(entity, px, py, &mut player_actions);
            }
        }

        if player_actions.len() > 0 {
            let player = self.entity_map.get_mut(&0)?;
            let mut delay = 0.0;
            let d_inc = 0.3;
            for action in player_actions {
                match action {
                    ecs::PickupAction::Health40p => {
                        let ci = player.combat_info.as_mut()?;
                        let h_inc = ci.max_health*4/10;
                        ci.health += h_inc;
                        if ci.health > ci.max_health {
                            ci.health = ci.max_health;
                        }

                        self.floating_texts.push(core::FloatingText::new(
                                h_inc.to_string(), "floating_green".to_string(), delay, px as f32, py as f32));
                        delay += d_inc;
                    },
                    ecs::PickupAction::Cognition40p => {
                        let ci = player.combat_info.as_mut()?;
                        let c_inc = ci.max_cognition*4/10;
                        ci.cognition += c_inc;
                        if ci.cognition > ci.max_cognition {
                            ci.cognition = ci.max_cognition;
                        }

                        combat_logic::update_vision(player);
                        let lp = player.logical_pos.as_ref()?;
                        let vi = player.vision_info.as_ref()?;
                        self.tile_grid.update_visibility(lp.x, lp.y, vi.radius);

                        self.floating_texts.push(core::FloatingText::new(
                                c_inc.to_string(), "floating_blue".to_string(), delay, px as f32, py as f32));
                        delay += d_inc;
                    },
                    _ => console_log!("No implementation for player action {:?}", action),
                };
            }
        }

        Some(())
    }

}

fn grab_entity(
    entity: &mut ecs::Entity,
    px: i32, py: i32,
    player_actions: &mut Vec<ecs::PickupAction>) -> Option<()> {

    let lp = entity.logical_pos.as_ref()?;
    if lp.x == px && lp.y == py {
        let pi = entity.pickup_info.as_ref()?;
        for action in &pi.actions {
            match action {
                ecs::PickupAction::Health40p => player_actions.push(action.clone()),
                ecs::PickupAction::Cognition40p => player_actions.push(action.clone()),
                ecs::PickupAction::Die => entity.dead = true,
            };
        }
    }

    Some(())
}

