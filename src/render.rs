use crate::{
    GameState,
    jsDrawImageFull,
    jsDrawImagePart,
};

use core::*;
use ecs::*;

pub fn draw_all(state: &GameState) {
    clear(state);
    draw_tiles(state);
    draw_entities(state);
}

fn clear(state: &GameState) {
    state.ctx.set_fill_style(&"black".into());
    state.ctx.fill_rect(0., 0., state.width as f64, state.height as f64);
}

fn draw_tiles(state: &GameState) {
    /*
    jsDrawImageFull(&state.ctx, "assets/prison_tiles.png", 0, 0);
    jsDrawImageFull(&state.ctx, "assets/target.png", 0, 0);
    jsDrawImageFull(&state.ctx, "assets/target.png", 64, 0);

    jsDrawImagePart(&state.ctx, "assets/prison_tiles.png", 0, 0, 32, 32, 100, 100, 32, 32);
    */

    let n_x: u32 = state.width / TILE_SIZE;
    let n_y: u32 = state.height / TILE_SIZE;
    for tx in 0..n_x {
        for ty in 0..n_y {
            if tx == 0 || tx == n_x - 1 || ty == 0 || ty == n_y - 1 {
                draw_tile(state, tx, ty, TileType::WALL);
            }
            else {
                draw_tile(state, tx, ty, TileType::FLOOR);
            }
        }
    }
}

fn draw_entities(state: &GameState) {
    match &state.player.draw_pos {
        Some(draw_pos) => draw_entity(state, draw_pos.x, draw_pos.y, "player_none"),
        None => {},
    };
}

fn draw_tile(state: &GameState, tx: u32, ty: u32, tile_type: TileType) {
    let x = tx * TILE_SIZE;
    let y = ty * TILE_SIZE;
    let (sx, sy) = match tile_type {
        TileType::FLOOR => (0, 0),
        TileType::WALL => (0, 32),
        TileType::DOORCLOSED => (32, 0),
        TileType::DOOROPEN => (32, 32),
    };
    jsDrawImagePart(&state.ctx, "prison_tiles",
                    sx, sy, 32, 32,
                    x as f32, y as f32, TILE_SIZE as f32, TILE_SIZE as f32);
}

fn draw_entity(state: &GameState, x: f32, y: f32, entity_name: &str) {
    jsDrawImagePart(&state.ctx, entity_name,
                    0, 0, 32, 32,
                    x, y, TILE_SIZE as f32, TILE_SIZE as f32);
}

