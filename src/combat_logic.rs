use crate::GameState;
use util::get_next_id;

use debug::log;

pub fn process_combat(state: &mut GameState) {
    let max_id = get_next_id(&state.entity_map);
    for id in 0..max_id {
        process_attack(state, id);
    }
}

fn process_attack(state: &mut GameState, id: usize) -> Option<()> {
    let tgt_id = state.entity_map.get(&id)?.combat_info.as_ref()?.current_attack?;

    let dmg = state.entity_map.get(&id)?.combat_info.as_ref()?.damage;
    state.entity_map.get_mut(&tgt_id)?.combat_info.as_mut()?.health -= dmg;

    if state.entity_map.get(&tgt_id)?.combat_info.as_ref()?.health <= 0 {
        console_log!("DEAD!");
        state.entity_map.remove(&tgt_id);
    }

    state.entity_map.get_mut(&id)?.combat_info.as_mut()?.current_attack = None;
    Some(())
}

