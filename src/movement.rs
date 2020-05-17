use core::MOVE_SPEED;
use ecs::Entity;

pub fn move_entity(entity: &mut Entity, frame_time: f32) -> Option<()> {
    let tx: f32 = entity.target.as_ref()?.x as f32;
    let ty: f32 = entity.target.as_ref()?.y as f32;

    if let Some(ref mut vp) = entity.visual_pos {
        let dd = MOVE_SPEED * frame_time;
        let dx = tx - vp.x;
        let dy = ty - vp.y;
        if dx.abs() > dd {
            vp.x += dd * sign(dx) as f32;
        }
        else {
            vp.x += dx;
        }
        if dy.abs() > dd {
            vp.y += dd * sign(dy) as f32;
        }
        else {
            vp.y += dy;
        }
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

