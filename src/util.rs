use std::collections::HashMap;
use rand::prelude::*;

use core::Camera;
use ecs::EntityId;

pub fn rand_usize(x0: usize, x1: usize) -> usize {
    random::<usize>() % (x1 - x0) + x0
}

pub fn world_to_pixel(w_x: f32, w_y: f32, camera: &Camera) -> (i32, i32) {
    //let p_x = (w_x - camera.x) as i32 * camera.tile_pix as i32 + (camera.canvas_width/2) as i32;
    //let p_y = (w_y - camera.y) as i32 * camera.tile_pix as i32 + (camera.canvas_height/2) as i32;
    let p_x = (w_x - camera.x) * camera.tile_pix as f32 + (camera.canvas_width/2) as f32;
    let p_y = (w_y - camera.y) * camera.tile_pix as f32 + (camera.canvas_height/2) as f32;
    (p_x as i32, p_y as i32)
}

pub fn pixel_to_world(p_x: i32, p_y: i32, camera: &Camera) -> (f32, f32) {
    let tile_pix_f: f32 = camera.tile_pix as f32;
    let w_x = (p_x as i32 - camera.canvas_width as i32/2) as f32/tile_pix_f + camera.x;
    let w_y = (p_y as i32 - camera.canvas_height as i32/2) as f32/tile_pix_f + camera.y;
    (w_x, w_y)
}

pub fn get_next_id<T>(hm: &HashMap<EntityId, T>) -> EntityId {
    /*let mut max_id: EntityId = 0;
    for &id in hm.keys() {
        if id > max_id {
            max_id = id;
        }
    }

    max_id + 1*/
    hm.keys().max().map(|i| i+1).unwrap_or(0)
}

