use ecs::*;
use util::*;

pub fn create_player(tile_size: u32) -> Entity {
    let tile_pos = CTilePos { x: 5, y: 5 };
    Entity {
        tile_pos: Some(tile_pos.clone()),
        draw_pos: Some(tile_to_draw(&tile_pos, tile_size)),
        target: None,
    }
}

