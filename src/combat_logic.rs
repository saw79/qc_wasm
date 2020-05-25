use crate::GameState;
use core::FloatingText;
use ecs::Entity;
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

    // basic damage hit
    let dmg = state.entity_map.get(&id)?.combat_info.as_ref()?.damage;
    state.entity_map.get_mut(&tgt_id)?.combat_info.as_mut()?.health -= dmg;
    state.entity_map.get_mut(&tgt_id)?.combat_info.as_mut()?.cognition -= dmg;

    // cognition
    update_vision(state.entity_map.get_mut(&tgt_id)?);
    if id == 0 {
        state.update_visibility();
    }

    // check death
    if state.entity_map.get(&tgt_id)?.combat_info.as_ref()?.health <= 0 {
        state.entity_map.get_mut(&tgt_id)?.dead = true;
    }

    state.entity_map.get_mut(&id)?.combat_info.as_mut()?.current_attack = None;

    let tgt_ri = state.entity_map.get(&tgt_id)?.render_info.as_ref()?;
    state.floating_texts.push(FloatingText::new(dmg.to_string(), "floating_red".to_string(), 0.0, tgt_ri.x, tgt_ri.y));

    Some(())
}

pub fn update_vision(entity: &mut Entity) -> Option<()> {
    let ci = entity.combat_info.as_ref()?;
    let vi = entity.vision_info.as_mut()?;

    vi.radius = vi.max_radius * ci.cognition / ci.max_cognition;
    let min_rad = if entity.is_player { 2 } else { 1 };
    if vi.radius < min_rad {
        vi.radius = min_rad;
    }

    Some(())
}

