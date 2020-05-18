use core::{TileType, TileGrid};
use ecs::{Entity, Action};

use rand::prelude::*;

pub fn compute_action(entity: &Entity, tile_grid: &TileGrid) -> Option<Action> {
    let entity_pos = entity.logical_pos.as_ref()?;

    let valid_moves = [(-1, -1), (0, -1), (1, -1), (-1, 0), (0, 0), (1, 0), (-1, 1), (0, 1), (1, 1)]
        .iter()
        .map(|(dx, dy)| (entity_pos.x as i32 + dx, entity_pos.y as i32 + dy))
        .filter(|(x, y)| tile_grid.at(*x, *y) != &TileType::WALL)
        .collect::<Vec<(i32, i32)>>();

    let idx = (random::<u32>() % valid_moves.len() as u32) as usize;
    let (x, y) = valid_moves[idx];

    Some(Action::Move(x as u32, y as u32))
}

