use crate::GameState;
use core::Direction;
use tile_grid::TileGrid;
use ecs::{EntityId, Entity, LogicalPos, Action, AlertState};
use factory::{get_walk_anim};
use path_logic;
use ai_logic;

use debug::log;

enum GetTurnResult {
    Blocked,
    Success,
    Failure,
}

fn get_player_pos(state: &GameState) -> Option<(i32, i32)> {
    let player = state.entity_map.get(&0)?;
    let lp = player.logical_pos.as_ref()?;
    Some((lp.x, lp.y))
}

pub fn process_turns(state: &mut GameState) {
    loop {
        let (pl_x, pl_y) = match get_player_pos(state) {
            Some((x, y)) => (x, y),
            None => {
                console_log!("[ERROR] (turn_logic/process_turns) - can't get player stuff!");
                return;
            },
        };

        match process_turn(state, pl_x, pl_y) {
            Some(GetTurnResult::Blocked) => return, // break, waiting for animation or input
            Some(GetTurnResult::Success) => state.turn_queue.inc(),
            Some(GetTurnResult::Failure) => console_log!("turn_logic::process_turns failure!!!"),
            None                         => console_log!("turn_logic::process_turns failure!!!"),
        };
    }
}

fn process_turn(state: &mut GameState, pl_x: i32, pl_y: i32) -> Option<GetTurnResult> {
    let id = state.turn_queue.get();

    if let Some(aq) = state.entity_map.get(&id)?.action_queue.as_ref() {
        if let Some(_) = aq.current {
            // last turn still active, wait for it to be removed
            Some(GetTurnResult::Blocked)
        } else {
            // compute NEW turn
            if id > 0 {
                update_alertness(state.entity_map.get_mut(&id)?, &state.tile_grid, pl_x, pl_y);
            }

            match get_turn_action(state, id, pl_x, pl_y) {
                Some(action) => {
                    let entity = state.entity_map.get_mut(&id)?;
                    perform_action_logic(entity, action, &mut state.tile_grid, id == 0, pl_x, pl_y);
                    // update alertness again
                    if id > 0 {
                        update_alertness(entity, &state.tile_grid, pl_x, pl_y);
                    }

                    Some(GetTurnResult::Success)
                },
                None => Some(GetTurnResult::Blocked),
            }
        }
    } else {
        // entity does not have ActionQueue comp, should not be in turn queue!
        console_log!("ERROR: turn queue entity doesn't have actionqueue!!!!");
        Some(GetTurnResult::Failure)
    }
}

fn get_turn_action(state: &mut GameState, id: EntityId, pl_x: i32, pl_y: i32) -> Option<Action> {
    // ActionQueue component is REQUIRED
    let aq = state.entity_map.get(&id)?.action_queue.as_ref()?;

    /* logic is:
     * 1. pull from ActionQueue
     * 2. pull from EntityTarget
     * 3. block for human / AI for computer
     */

    if aq.queue.len() > 0 {
        Some(state.entity_map.get_mut(&id)?.action_queue.as_mut()?.queue.remove(0))
    } else if let Some(tgt) = state.entity_map.get(&id)?.entity_target.as_ref() {
        let x = try_target_path(
            state.entity_map.get(&id)?, state.entity_map.get(&tgt.id)?, &state.tile_grid);
        if let Some(Action::Move(_, _)) = x {
            x
        } else {
            state.entity_map.get_mut(&id)?.entity_target = None;
            x
        }
    } else if id > 0 {
        ai_logic::compute_action(state.entity_map.get(&id)?, &state.tile_grid, pl_x, pl_y)
    } else {
        None
    }
}

fn try_target_path(entity: &Entity, tgt: &Entity, tile_grid: &TileGrid) -> Option<Action> {
    let lp = entity.logical_pos.as_ref()?;
    let tgt_lp = tgt.logical_pos.as_ref()?;

    match path_logic::get_path(lp.x, lp.y, tgt_lp.x, tgt_lp.y, &tile_grid) {
        Some((path, _cost)) => {
            if path.len() > 2 {
                Some(Action::Move(path[1].0, path[1].1))
            } else if path.len() == 2 {
                Some(Action::Attack(entity.entity_target.as_ref()?.id))
            } else {
                None
            }
        },
        None => None,
    }
}

fn get_direction(x0: i32, y0: i32, x1: i32, y1: i32) -> Direction {
    if x1 - x0 > 0 {
        Direction::Right
    }
    else if x1 - x0 < 0 {
        Direction::Left
    }
    else if y1 - y0 > 0 {
        Direction::Down
    }
    else {
        Direction::Up
    }
}

fn perform_action_logic(entity: &mut Entity, action: Action, tile_grid: &mut TileGrid,
                        is_player: bool, pl_x: i32, pl_y: i32) -> Option<()> {
    match action {
        Action::Wait => {},//entity.action_queue.as_mut()?.current = None,
        Action::Move(move_x, move_y) => {
            // set animation direction based on move direction
            let logical = entity.logical_pos.as_ref()?;
            let dir_opt = if let Some(ri) = entity.render_info.as_mut() {
                let dir = get_direction(logical.x, logical.y, move_x, move_y);
                ri.frames = get_walk_anim(entity.name, &dir);
                Some(dir)
            } else {
                None
            };

            // PROCESS
            // set logical position to desired move pos
            entity.logical_pos = Some(LogicalPos { x: move_x, y: move_y });
            // set enemy vision wedge direction
            if let Some(vi) = entity.vision_info.as_mut() {
                match dir_opt {
                    Some(dir) => vi.dir = dir,
                    None => {},
                };
            }

            if is_player {
                let radius = entity.vision_info.as_ref()?.radius;
                tile_grid.update_visibility(move_x, move_y, radius);
            }
            
            entity.action_queue.as_mut()?.current = Some(action);
        },
        Action::Attack(id) => {
            entity.combat_info.as_mut()?.current_attack = Some(id);
            let lp = entity.logical_pos.as_ref()?;
            let dir = get_direction(lp.x, lp.y, pl_x, pl_y);
            if let Some(ri) = entity.render_info.as_mut() {
                ri.frames = get_walk_anim(entity.name, &dir);
            }
            entity.vision_info.as_mut()?.dir = dir;
        },
        Action::Look(dir) => {
            entity.vision_info.as_mut()?.dir = dir;
        },
    };

    Some(())
}

fn update_alertness(entity: &mut Entity, tile_grid: &TileGrid, pl_x: i32, pl_y: i32) -> Option<()> {
    let lp = entity.logical_pos.as_ref()?;
    let vi = entity.vision_info.as_mut()?;
    if tile_grid.visibility_from_to(lp.x, lp.y, pl_x, pl_y, vi.radius, Some(&vi.dir)) {
        vi.last_location = (pl_x, pl_y);
        vi.alert_state = AlertState::KILL;
    } else {
        match vi.alert_state {
            AlertState::KILL => vi.alert_state = AlertState::SEARCH,
            AlertState::SEARCH => vi.alert_state =
                if (lp.x, lp.y) == vi.last_location {
                    AlertState::PATROL
                } else {
                    AlertState::SEARCH
                },
            AlertState::PATROL => vi.alert_state = AlertState::PATROL,
        };
    }

    Some(())
}

