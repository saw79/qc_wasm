use core::*;
use ecs::*;
use util::*;

pub fn create_player() -> Entity {
    let tile_pos = CTilePos { x: 5, y: 5 };
    Entity {
        tile_pos: Some(tile_pos.clone()),
        draw_pos: Some(tile_to_draw(&tile_pos)),
        target: None,
    }
}

