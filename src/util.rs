use core::Camera;

pub fn world_to_pixel(w_x: f32, w_y: f32, camera: &Camera) -> (f32, f32) {
    let tile_pix: f32 = camera.tile_pix as f32;
    let p_x = (w_x - camera.x)*tile_pix + (camera.canvas_width as f32)/2.0;
    let p_y = (w_y - camera.y)*tile_pix + (camera.canvas_height as f32)/2.0;
    (p_x, p_y)
}

pub fn pixel_to_world(p_x: f32, p_y: f32, camera: &Camera) -> (f32, f32) {
    let tile_pix: f32 = camera.tile_pix as f32;
    let w_x = (p_x - (camera.canvas_width as f32)/2.0)/tile_pix + camera.x;
    let w_y = (p_y - (camera.canvas_height as f32)/2.0)/tile_pix + camera.y;
    (w_x, w_y)
}

