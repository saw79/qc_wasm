use core::*;
use ecs::{CTilePos, CDrawPos};

pub fn tile_to_draw(tile_pos: &CTilePos) -> CDrawPos {
    CDrawPos {
        x: (tile_pos.x * get_tile_size()) as f32,
        y: (tile_pos.y * get_tile_size()) as f32,
    }
}

