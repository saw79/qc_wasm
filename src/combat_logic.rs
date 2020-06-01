use crate::GameState;
use core::FloatingText;
use ecs::{EntityId, Entity};

use debug::log;

pub fn process_combat(state: &mut GameState) {
    let keys: Vec<EntityId> = state.entity_map.keys().cloned().collect();
    for id in keys {
        process_attack(state, id);
    }
}

fn process_attack(state: &mut GameState, id: EntityId) -> Option<()> {
    let tgt_id = state.entity_map.get(&id)?.combat_info.as_ref()?.current_attack?;

    // basic damage hit
    let dmg = state.entity_map.get(&id)?.combat_info.as_ref()?.damage;
    state.entity_map.get_mut(&tgt_id)?.combat_info.as_mut()?.health -= dmg;
    state.entity_map.get_mut(&tgt_id)?.combat_info.as_mut()?.cognition -= dmg;
    if state.entity_map.get(&tgt_id)?.combat_info.as_ref()?.cognition <= 0 {
        state.entity_map.get_mut(&tgt_id)?.combat_info.as_mut()?.cognition = 0;
    }

    // cognition
    update_vision(state.entity_map.get_mut(&tgt_id)?);
    if tgt_id == 0 {
        state.update_visibility();
    }

    // check death
    if state.entity_map.get(&tgt_id)?.combat_info.as_ref()?.health <= 0 {
        state.entity_map.get_mut(&tgt_id)?.dead = true;
    }

    state.entity_map.get_mut(&id)?.combat_info.as_mut()?.current_attack = None;

    let tgt_lp = state.entity_map.get(&tgt_id)?.logical_pos.as_ref()?;
    state.floating_texts.push(FloatingText::new(dmg.to_string(), "floating_red".to_string(), 0.0, tgt_lp.x as f32, tgt_lp.y as f32));

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

