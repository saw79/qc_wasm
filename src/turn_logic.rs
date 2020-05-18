use crate::GameState;
use core::TileGrid;
use ecs::{Entity, LogicalPos, Action};

use debug::log;

pub fn compute_turns(state: &mut GameState) {
    let mut entity = if state.curr_turn == -1 {
        &mut state.player
    } else {
        &mut state.enemies[state.curr_turn as usize]
    };

    if let Some(aq) = &entity.action_queue {
        if let Some(_) = aq.current {
            // last turn still active, wait for it to be removed
            return;
        } else {
            // compute a turn!
            // Some -> computed new turn successfully
            // None -> blocking, waiting for input, etc...
            match compute_turn(&mut entity, &state.tile_grid) {
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
    if state.curr_turn == state.enemies.len() as i32 {
        state.curr_turn = -1;
    }
}

fn compute_turn<>(entity: &mut Entity, _tile_grid: &TileGrid) -> Option<Action> {
    if let Some(ref mut aq) = &mut entity.action_queue {
        if aq.queue.len() > 0 {
            aq.current = Some(aq.queue.remove(0));
            return aq.current.clone();
        }
    }
    
    None
}

fn perform_action_logic(entity: &mut Entity, action: &Action) {
    match action {
        Action::Move(wx, wy) => entity.logical_pos = Some(LogicalPos { x: *wx, y: *wy }),
        _ => console_log!("UNIMPLEMENTED ACTION"),
    };
}

