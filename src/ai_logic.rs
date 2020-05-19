use core::{TileType, TileGrid};
use ecs::{Entity, Action};

use rand::prelude::*;

pub fn compute_action(entity: &Entity, tile_grid: &TileGrid) -> Option<Action> {
    let lp = entity.logical_pos.as_ref()?;

    let valid_moves = [(-1, -1), (0, -1), (1, -1), (-1, 0), (0, 0), (1, 0), (-1, 1), (0, 1), (1, 1)]
        .iter()
        .map(|(dx, dy)| (lp.x + dx, lp.y + dy))
        .filter(|(x, y)| tile_grid.at(*x, *y) != &TileType::WALL)
        .collect::<Vec<(i32, i32)>>();

    let idx = (random::<u32>() % valid_moves.len() as u32) as usize;
    let (x, y) = valid_moves[idx];

    Some(Action::Move(x, y))
}

