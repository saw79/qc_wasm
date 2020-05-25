use std::f64;

use web_sys::CanvasRenderingContext2d;

use crate::{
    GameState,
    jsDrawImage,
    jsDrawImageAlpha,
    jsAlphaToMain,
    jsDrawString,
};

use core::{Direction, Camera};
use tile_grid::{TileType, Visibility, TileGrid};
use util::world_to_pixel;
use ecs::{Entity, AlertState};

use debug::log;

pub fn draw_all(state: &GameState) {
    clear(state);
    draw_tiles(state);
    jsAlphaToMain(&state.ctx, &state.ctx_alpha, 0.5);
    draw_entities(state);
    draw_floating_text(state);
}

fn clear(state: &GameState) {
    state.ctx.set_fill_style(&"black".into());
    state.ctx.fill_rect(
        0.0, 0.0, state.camera.canvas_width as f64, state.camera.canvas_height as f64);

    // clear special alpha context
    state.ctx_alpha.set_fill_style(&"rgba(0, 0, 0, 0)".into());
    state.ctx_alpha.clear_rect(
        0.0, 0.0, state.camera.canvas_width as f64, state.camera.canvas_height as f64);
}

fn draw_tiles(state: &GameState) {
    let tile_x_0 = (state.camera.x - state.camera.width/2.0) as i32 - 1;
    let tile_x_1 = (state.camera.x + state.camera.width/2.0) as i32 + 1;
    let tile_y_0 = (state.camera.y - state.camera.height/2.0) as i32 - 1;
    let tile_y_1 = (state.camera.y + state.camera.height/2.0) as i32 + 1;

    for ix in tile_x_0..tile_x_1+1 {
        for iy in tile_y_0..tile_y_1+1 {
            if ix >= 0 && ix < state.tile_grid.width as i32
                && iy >= 0 && iy < state.tile_grid.height as i32 {

                let vis = state.tile_grid.get_visibility(ix, iy);
                if vis != &Visibility::UNSEEN {
                    draw_tile(state, ix, iy, state.tile_grid.at(ix, iy), vis);
                }
            }
        }
    }
}

fn draw_entities(state: &GameState) {
    for (id, entity) in &state.entity_map {
        draw_entity(&state.ctx, entity, &state.camera, &state.tile_grid, *id > 0);
    }
}

fn draw_tile(state: &GameState, ix: i32, iy: i32, tile_type: &TileType, vis: &Visibility) {
    let (px, py) = world_to_pixel(ix as f32, iy as f32, &state.camera);
    let (sx, sy) = match tile_type {
        TileType::EMPTY => return,
        TileType::FLOOR => (0, 0),
        TileType::WALL => (0, 32),
        TileType::DOORCLOSED => (32, 0),
        TileType::DOOROPEN => (32, 32),
    };

    if vis == &Visibility::VISIBLE {
        jsDrawImage(&state.ctx, "prison_tiles",
                    sx, sy, 32, 32,
                    px-1, py-1, state.camera.tile_pix+2, state.camera.tile_pix+2);
    } else {
        jsDrawImage(&state.ctx_alpha, "prison_tiles",
                    sx, sy, 32, 32,
                    px-1, py-1, state.camera.tile_pix+2, state.camera.tile_pix+2);
    }
}

//fn draw_entity(state: &GameState, ri: &RenderInfo) {
fn draw_entity(ctx: &CanvasRenderingContext2d, entity: &Entity,
               camera: &Camera, tile_grid: &TileGrid, is_enemy: bool)
    -> Option<()> {
    let ri = entity.render_info.as_ref()?;

    if tile_grid.get_visibility(ri.x as i32, ri.y as i32) != &Visibility::VISIBLE {
        if let Some(_) = entity.combat_info.as_ref() {
            return Some(());
        }

        if tile_grid.get_visibility(ri.x as i32, ri.y as i32) == &Visibility::UNSEEN {
            return Some(());
        }
    }

    let (px, py) = world_to_pixel(ri.x, ri.y, camera);

    // vision
    if let Some(vi) = entity.vision_info.as_ref() {
        let vi_size: i32 = (2*vi.radius + 1) * camera.tile_pix;

        if vi.is_wedge {
            let angle = match vi.dir {
                Direction::Right => 0.0,
                Direction::Down => f64::consts::PI/2.0,
                Direction::Left => f64::consts::PI*2.0/2.0,
                Direction::Up => f64::consts::PI*3.0/2.0,
            };

            ctx.save();
            let (center_x, center_y) = (px + camera.tile_pix/2, py + camera.tile_pix/2);
            if let Err(_) = ctx.translate(center_x as f64, center_y as f64) {
                ctx.restore();
                return None;
            }
            if let Err(_) = ctx.rotate(angle) {
                ctx.restore();
                return None;
            }

            jsDrawImageAlpha(ctx, "vision_wedge",
                             0, 0, 256, 256,
                             -vi_size/2, -vi_size/2, vi_size, vi_size,
                             0.05);

            ctx.restore();
        } else {
            console_log!("Vision Info rendering for !is_wedge not yet implemented!!!");
        }
    }

    // entity
    if ri.curr_frame < ri.frames.len() {
        let rf = &ri.frames[ri.curr_frame];
        jsDrawImage(ctx, rf.sheet_name,
                    rf.sheet_x, rf.sheet_y, rf.sheet_w, rf.sheet_h,
                    px, py, camera.tile_pix, camera.tile_pix);
    }

    // health bar
    if is_enemy {
        let ci = entity.combat_info.as_ref()?;
        let hbw = (ci.health as f32 / ci.max_health as f32 * camera.tile_pix as f32) as i32;
        jsDrawImage(ctx, "health_bar",
                    0, 0, 100, 20,
                    px+(camera.tile_pix-hbw)/2, py-camera.tile_pix/20, hbw, camera.tile_pix/10);
    }

    // alertness state
    if is_enemy {
        if let Some(vi) = entity.vision_info.as_ref() {
            match vi.alert_state {
                AlertState::PATROL => jsDrawString(ctx, "patrol", "...",
                                                   px + camera.tile_pix/2, py - camera.tile_pix/10),
                AlertState::SEARCH => jsDrawString(ctx, "search", "?",
                                                   px + camera.tile_pix/2, py - camera.tile_pix/10),
                AlertState::KILL   => jsDrawString(ctx, "kill", "!",
                                                   px + camera.tile_pix/2, py - camera.tile_pix/10),
            };
        }
    }

    Some(())
}

fn draw_floating_text(state: &GameState) {
    for ft in &state.floating_texts {
        if ft.curr_time >= 0.0 {
            let (px, py) = world_to_pixel(ft.x, ft.y, &state.camera);
            jsDrawString(&state.ctx, &ft.style, &ft.text, px, py);
        }
    }
}

