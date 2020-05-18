use crate::GameState;
use ecs::Entity;

pub fn compute_animations(state: &mut GameState, dt: f32) {
    for entity in state.entities.iter_mut() {
        compute_animation(entity, dt);
    }
}

fn compute_animation(entity: &mut Entity, dt: f32) -> Option<()> {
    let ri = entity.render_info.as_mut()?;
    if ri.active {
        ri.time += dt;
        ri.curr_frame = ((ri.time / ri.frame_duration) as usize) % ri.frames.len();
    }

    Some(())
}

