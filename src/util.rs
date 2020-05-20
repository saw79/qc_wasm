use std::collections::HashMap;

use core::Camera;

pub fn world_to_pixel(w_x: f32, w_y: f32, camera: &Camera) -> (u32, u32) {
    //let p_x = (w_x - camera.x) as i32 * camera.tile_pix as i32 + (camera.canvas_width/2) as i32;
    //let p_y = (w_y - camera.y) as i32 * camera.tile_pix as i32 + (camera.canvas_height/2) as i32;
    let p_x = (w_x - camera.x) * camera.tile_pix as f32 + (camera.canvas_width/2) as f32;
    let p_y = (w_y - camera.y) * camera.tile_pix as f32 + (camera.canvas_height/2) as f32;
    (p_x as u32, p_y as u32)
}

pub fn pixel_to_world(p_x: u32, p_y: u32, camera: &Camera) -> (f32, f32) {
    let tile_pix_f: f32 = camera.tile_pix as f32;
    let w_x = (p_x as i32 - camera.canvas_width as i32/2) as f32/tile_pix_f + camera.x;
    let w_y = (p_y as i32 - camera.canvas_height as i32/2) as f32/tile_pix_f + camera.y;
    (w_x, w_y)
}

pub fn get_next_id<T>(hm: &HashMap<usize, T>) -> usize {
    let mut max_id: usize = 0;
    for &id in hm.keys() {
        if id > max_id {
            max_id = id;
        }
    }

    max_id + 1
}

