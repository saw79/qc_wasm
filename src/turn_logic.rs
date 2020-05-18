use crate::GameState;
use core::TileGrid;
use ecs::{Entity, LogicalPos, Action};
use factory::{Direction, get_walk_anim};
use ai_logic;

use debug::log;

pub fn compute_turns(state: &mut GameState) {
    let ref mut entity = &mut state.entities[state.curr_turn];
    let is_player = state.curr_turn == 0;

    if let Some(aq) = &entity.action_queue {
        if let Some(_) = aq.current {
            // last turn still active, wait for it to be removed
            return;
        } else {
            // compute a turn!
            // Some -> computed new turn successfully
            // None -> blocking, waiting for input, etc...
            match compute_turn(entity, &state.tile_grid, is_player) {
                Some(action) => {
                    perform_action_logic(entity, &action);
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
}

fn increment_turn(state: &mut GameState) {
    state.curr_turn += 1;
    if state.curr_turn == state.entities.len() {
        state.curr_turn = 0;
    }
}

fn compute_turn(entity: &mut Entity, tile_grid: &TileGrid, is_player: bool) -> Option<Action> {
    // ActionQueue component is REQUIRED
    let aq = entity.action_queue.as_ref()?;

    // 1. get_next_action
    let action = if aq.queue.len() > 0 {
        entity.action_queue.as_mut()?.queue.remove(0)
    } else if !is_player {
        match ai_logic::compute_action(entity, tile_grid) {
            Some(a) => a,
            None => return None,
        }
    } else {
        return None;
    };

    // 2. process_action
    match action {
        Action::Move(mx, my) => {
            let logical = entity.logical_pos.as_ref()?;
            let dx: i32 = (mx - logical.x) as i32;
            let dy: i32 = (my - logical.y) as i32;
            if let Some(ri) = entity.render_info.as_mut() {
                if dx > 0 {
                    ri.frames = get_walk_anim(entity.name, &Direction::Right);
                }
                else if dx < 0 {
                    ri.frames = get_walk_anim(entity.name, &Direction::Left);
                }
                else { // dx == 0
                    if dy > 0 {
                        ri.frames = get_walk_anim(entity.name, &Direction::Down);
                    }
                    else {
                        ri.frames = get_walk_anim(entity.name, &Direction::Up);
                    }
                }
            }
        },
        _ => {},
    };

    let aq = entity.action_queue.as_mut()?;
    aq.current = Some(action);
    return aq.current.clone();
}

fn perform_action_logic(entity: &mut Entity, action: &Action) {
    match action {
        Action::Move(wx, wy) => entity.logical_pos = Some(LogicalPos { x: *wx, y: *wy }),
        _ => console_log!("UNIMPLEMENTED ACTION"),
    };
}

