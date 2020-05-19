use crate::GameState;
use core::MOVE_SPEED;
use ecs::{Entity, Action, ActionQueue};

pub fn move_entities(state: &mut GameState, dt: f32) {
    for (&id, entity) in state.entity_map.iter_mut() {
        match move_entity(entity, dt) {
            Some((x, y)) if id == 0 => {
                state.camera.x = x;
                state.camera.y = y;
            },
            _ => {},
        };
    }
}

fn move_entity(entity: &mut Entity, dt: f32) -> Option<(f32, f32)> {
    let action_queue: &mut ActionQueue = entity.action_queue.as_mut()?;

    match action_queue.current {
        Some(Action::Move(tx, ty)) =>  {
            let tx = tx as f32;
            let ty = ty as f32;
            if let Some(ref mut ri) = entity.render_info {
                ri.active = true;
                let dd = MOVE_SPEED * dt;
                let dx = tx - ri.x;
                let dy = ty - ri.y;

                let mut finished = true;

                if dx.abs() > dd {
                    ri.x += dd * sign(dx) as f32;
                    finished = false;
                }
                else {
                    ri.x += dx;
                }
                if dy.abs() > dd {
                    ri.y += dd * sign(dy) as f32;
                    finished = false;
                }
                else {
                    ri.y += dy;
                }

                if finished {
                    action_queue.current = None;
                    ri.active = false;
                }

                Some((ri.x, ri.y))
            } else {
                None
            }
        },
        _ => None
    }
}

fn sign(x: f32) -> i32 {
    if x < 0.0 {
        -1
    }
    else {
        1
    }
}

