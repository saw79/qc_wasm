use tile_grid::{TileType, TileGrid};
use ecs::{Entity, Action, AlertState};
use path_logic::get_path;

use rand::prelude::*;

/*
 * Basic enemy logic:
 * 1. KILL:
 *      move/attack
 * 2. SEARCH:
 *      a. go to last known location
 *      b. spin around?
 * 3. PATROL:
 *      move randomly
 */

pub fn compute_action(entity: &Entity, tile_grid: &TileGrid, player: &Entity) -> Option<Action> {
    if let Some(vi) = entity.vision_info.as_ref() {
        match vi.alert_state {
            AlertState::PATROL => compute_action_patrol(entity, tile_grid),
            AlertState::SEARCH => compute_action_search(entity, tile_grid),
            AlertState::KILL => compute_action_kill(entity, tile_grid, player),
        }
    } else {
        compute_action_patrol(entity, tile_grid)
    }
}

pub fn compute_action_kill(entity: &Entity, tile_grid: &TileGrid, player: &Entity) -> Option<Action> {
    let (x0, y0) = {
        let lp = entity.logical_pos.as_ref()?;
        (lp.x, lp.y)
    };
    let (x1, y1) = {
        let lp = player.logical_pos.as_ref()?;
        (lp.x, lp.y)
    };

    if (x1-x0).abs() <= 1 && (y1-y0).abs() <= 1 {
        Some(Action::Attack(0))
    } else {
        get_move_toward(entity, tile_grid, x1, y1)
    }
}

pub fn compute_action_search(entity: &Entity, tile_grid: &TileGrid) -> Option<Action> {
    let (x1, y1) = entity.vision_info.as_ref()?.last_location;
    get_move_toward(entity, tile_grid, x1, y1)
}

pub fn compute_action_patrol(entity: &Entity, tile_grid: &TileGrid) -> Option<Action> {
    let lp = entity.logical_pos.as_ref()?;

    let valid_moves = [(-1, -1), (0, -1), (1, -1), (-1, 0), (0, 0), (1, 0), (-1, 1), (0, 1), (1, 1)]
        .iter()
        .map(|(dx, dy)| (lp.x + dx, lp.y + dy))
        .filter(|(x, y)|
                tile_grid.at(*x, *y) != &TileType::WALL &&
                tile_grid.at(*x, *y) != &TileType::EMPTY)
        .collect::<Vec<(i32, i32)>>();

    let idx = (random::<u32>() % valid_moves.len() as u32) as usize;
    let (x, y) = valid_moves[idx];

    Some(Action::Move(x, y))
}

fn get_move_toward(entity: &Entity, tile_grid: &TileGrid, tgt_x: i32, tgt_y: i32) -> Option<Action> {
    let lp = entity.logical_pos.as_ref()?;
    match get_path(lp.x, lp.y, tgt_x, tgt_y, tile_grid) {
        Some((path, _cost)) => {
            if path.len() > 1 {
                Some(Action::Move(path[1].0, path[1].1))
            } else if path.len() == 2 {
                Some(Action::Attack(0))
            } else {
                Some(Action::Wait)
            }
        },
        None => Some(Action::Wait),
    }
}

