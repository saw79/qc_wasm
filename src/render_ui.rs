use crate::{
    GameState,
    jsDrawImage,
};

pub fn draw_ui(state: &GameState) -> Option<()> {
    let ci = state.entity_map.get(&0)?.combat_info.as_ref()?;
    draw_status_bar(state, ci.health.abs() as u32, ci.max_health.abs() as u32, true);
    draw_status_bar(state, ci.cognition.abs() as u32, ci.max_cognition.abs() as u32, false);
    Some(())
}

fn draw_status_bar(state: &GameState, value: u32, max_value: u32, is_health: bool) {
    // for now lets say canvas_width = 100 hp
    let tile_pix = state.camera.tile_pix;
    let bar_width_max: u32 = (max_value as f32 / 100.0 * state.camera.canvas_width as f32) as u32;
    let bar_width_cur: u32 = (value as f32 / 100.0 * state.camera.canvas_width as f32) as u32;

    let x_pos = 10;
    let y_pos = 10 + if is_health { 0 } else { tile_pix/2 };
    let fill = if is_health { "health_fill" } else { "cog_fill" };

    draw_as_3patch(state, "status_bg",
                   11, 0, 0, 74, 40, x_pos, y_pos, bar_width_max, tile_pix/2);

    jsDrawImage(&state.ctx, fill,
                0, 0, 74, 40,
                x_pos, y_pos, bar_width_cur, tile_pix/2);

    draw_as_3patch(state, "status_cover",
                   11, 0, 0, 74, 40, x_pos, y_pos, bar_width_max, tile_pix/2);
}

fn draw_as_3patch(state: &GameState, name: &str, edge: u32,
                  sx: u32, sy: u32, sw: u32, sh: u32,
                  x: u32, y: u32, w: u32, h: u32) {
    jsDrawImage(&state.ctx, name,
                sx, sy, edge, sh,
                x, y, edge, h);
    jsDrawImage(&state.ctx, name,
                sx+edge, sy, sw-2*edge, sh,
                x+edge, y, w-2*edge, h);
    jsDrawImage(&state.ctx, name,
                sx+sw-edge, sy, edge, sh,
                x+w-edge, y, edge, h);
}

//status bars are 74 x 40

/*
Experience bar

float imHeight = Constants.HEALTH_BAR_SIZE / 6;
experienceImage = new Image(resourceManager.findRegion("experience_bar"));
experienceImage.setBounds(0, height - imHeight*.75f, width*experiencePercent, imHeight/2);
*/

