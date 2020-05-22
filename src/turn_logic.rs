use crate::GameState;
use core::Direction;
use tile_grid::TileGrid;
use util::get_next_id;
use ecs::{Entity, LogicalPos, Action, AlertState};
use factory::{get_walk_anim};
use path_logic;
use ai_logic;

use debug::log;

pub fn compute_turns(state: &mut GameState) -> Option<()> {
    if let None = state.entity_map.get(&state.curr_turn) {
        increment_turn(state);
    }

    if let Some(aq) = &state.entity_map.get(&state.curr_turn)?.action_queue {
        if let Some(_) = aq.current {
            // last turn still active, wait for it to be removed
            return Some(());
        } else {
            // compute NEW turn
            // update alertness
            if state.curr_turn > 0 {
                let (pl_x, pl_y) = {
                    let lp = state.entity_map.get(&0)?.logical_pos.as_ref()?;
                    (lp.x, lp.y)
                };
                update_alertness(
                    state.entity_map.get_mut(&state.curr_turn)?,
                    &state.tile_grid,
                    pl_x, pl_y);
            }
            match compute_turn(state) {
                Some(action) => {
                    perform_action_logic(
                        state.entity_map.get_mut(&state.curr_turn)?,
                        action,
                        &mut state.tile_grid,
                        state.curr_turn == 0);

                    // update alertness again
                    if state.curr_turn > 0 {
                        let (pl_x, pl_y) = {
                            let lp = state.entity_map.get(&0)?.logical_pos.as_ref()?;
                            (lp.x, lp.y)
                        };
                        update_alertness(
                            state.entity_map.get_mut(&state.curr_turn)?,
                            &state.tile_grid,
                            pl_x, pl_y);
                    }

                    increment_turn(state);
                },
                None => {},
            };
        }
    } else {
        // entity does not have ActionQueue comp which means we skip it
        // (it doesn't participate in this system)
        increment_turn(state);
    }

    Some(())
}

fn increment_turn(state: &mut GameState) {
    state.curr_turn += 1;
    if let None = state.entity_map.get(&state.curr_turn) {
        if state.curr_turn >= get_next_id(&state.entity_map) {
            state.curr_turn = 0;
        }
    }
}

fn compute_turn(state: &mut GameState) -> Option<Action> {
    // ActionQueue component is REQUIRED
    let id = state.curr_turn;
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
        ai_logic::compute_action(
            state.entity_map.get(&id)?,
            &state.tile_grid,
            state.entity_map.get(&0)?)
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

fn perform_action_logic(entity: &mut Entity, action: Action, tile_grid: &mut TileGrid, is_player: bool) -> Option<()> {
    match action {
        Action::Wait => {},//entity.action_queue.as_mut()?.current = None,
        Action::Move(move_x, move_y) => {
            // set animation direction based on move direction
            let logical = entity.logical_pos.as_ref()?;
            let dx: i32 = (move_x - logical.x) as i32;
            let dy: i32 = (move_y - logical.y) as i32;
            let dir_opt = if let Some(ri) = entity.render_info.as_mut() {
                let dir = if dx > 0 {
                    Direction::Right
                }
                else if dx < 0 {
                    Direction::Left
                }
                else {
                    if dy > 0 {
                        Direction::Down
                    }
                    else {
                        Direction::Up
                    }
                };
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
                tile_grid.update_visibility(move_x, move_y);
            }
            
            entity.action_queue.as_mut()?.current = Some(action);
        },
        Action::Attack(id) => {
            entity.combat_info.as_mut()?.current_attack = Some(id);
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

