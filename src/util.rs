use core::TILE_SIZE;
use ecs::{CTilePos, CDrawPos};

pub fn tile_to_draw(tile_pos: &CTilePos) -> CDrawPos {
    CDrawPos {
        x: (tile_pos.x * TILE_SIZE) as f32,
        y: (tile_pos.y * TILE_SIZE) as f32,
    }
}

