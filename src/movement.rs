use core::*;
use ecs::*;

pub fn move_entity(entity: &mut Entity, frame_time: f32, tile_size: u32) -> Option<()> {
    let tx: f32 = entity.target.as_ref()?.x as f32;
    let ty: f32 = entity.target.as_ref()?.y as f32;
    let tx_pix = tx * tile_size as f32;
    let ty_pix = ty * tile_size as f32;

    if let Some(ref mut dp) = entity.draw_pos {
        let dd = MOVE_SPEED * tile_size as f32 * frame_time;
        let dx = tx_pix - dp.x;
        let dy = ty_pix - dp.y;
        if dx.abs() > dd {
            dp.x += dd * sign(dx) as f32;
        }
        else {
            dp.x += dx;
        }
        if dy.abs() > dd {
            dp.y += dd * sign(dy) as f32;
        }
        else {
            dp.y += dy;
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

