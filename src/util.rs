use core::*;
use ecs::{CTilePos, CDrawPos};

pub fn tile_to_draw(tile_pos: &CTilePos, tile_size: u32) -> CDrawPos {
    CDrawPos {
        x: (tile_pos.x * tile_size) as f32,
        y: (tile_pos.y * tile_size) as f32,
    }
}

