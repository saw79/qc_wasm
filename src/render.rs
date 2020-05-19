use crate::{
    GameState,
    jsDrawImage,
};

use core::*;
use util::*;
use ecs::*;

pub fn draw_all(state: &GameState) {
    clear(state);
    draw_tiles(state);
    draw_entities(state);
}

fn clear(state: &GameState) {
    state.ctx.set_fill_style(&"black".into());
    state.ctx.fill_rect(0., 0., state.camera.canvas_width as f64, state.camera.canvas_height as f64);
}

fn draw_tiles(state: &GameState) {
    let tile_x_0 = (state.camera.x - state.camera.width/2.0) as i32 - 1;
    let tile_x_1 = (state.camera.x + state.camera.width/2.0) as i32 + 1;
    let tile_y_0 = (state.camera.y - state.camera.height/2.0) as i32 - 1;
    let tile_y_1 = (state.camera.y + state.camera.height/2.0) as i32 + 1;

    for ix in tile_x_0..tile_x_1+1 {
        for iy in tile_y_0..tile_y_1+1 {
            if ix >= 0 && ix < state.tile_grid.width as i32 &&
               iy >= 0 && iy < state.tile_grid.height as i32 {
                draw_tile(state, ix as f32, iy as f32, state.tile_grid.at(ix, iy));
            }
        }
    }
}

fn draw_entities(state: &GameState) {
    for entity in state.entity_map.values() {
        if let Some(ri) = &entity.render_info {
            draw_entity(state, ri);
        }
    }
}

fn draw_tile(state: &GameState, wx: f32, wy: f32, tile_type: &TileType) {
    let (px, py) = world_to_pixel(wx, wy, &state.camera);
    let (sx, sy) = match tile_type {
        TileType::FLOOR => (0, 0),
        TileType::WALL => (0, 32),
        TileType::DOORCLOSED => (32, 0),
        TileType::DOOROPEN => (32, 32),
    };
    let tile_pix = state.camera.tile_pix;
    jsDrawImage(&state.ctx, "prison_tiles",
                sx, sy, 32, 32,
                px as f32, py as f32, tile_pix as f32, tile_pix as f32, true);
}

fn draw_entity(state: &GameState, ri: &RenderInfo) {
    let (px, py) = world_to_pixel(ri.x, ri.y, &state.camera);

    let tile_pix = state.camera.tile_pix;
    if ri.curr_frame < ri.frames.len() {
        let rf = &ri.frames[ri.curr_frame];
        jsDrawImage(&state.ctx, rf.sheet_name,
                    rf.sheet_x, rf.sheet_y, rf.sheet_w, rf.sheet_h,
                    px, py, tile_pix as f32, tile_pix as f32, false);
    }
}

