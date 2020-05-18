use core::MOVE_SPEED;
use ecs::{Entity, Action, ActionQueue};

pub fn move_entity(entity: &mut Entity, frame_time: f32) -> Option<()> {
    let action_queue: &mut ActionQueue = entity.action_queue.as_mut()?;

    match action_queue.current {
        Some(Action::Move(tx, ty)) =>  {
            let tx = tx as f32;
            let ty = ty as f32;
            if let Some(ref mut ri) = entity.render_info {
                let dd = MOVE_SPEED * frame_time;
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
                }
            }
        },
        _ => {},
    }

    Some(())
}

fn sign(x: f32) -> i32 {
    if x < 0.0 {
        -1
    }
    else {
        1
    }
}

