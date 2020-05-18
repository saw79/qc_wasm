use crate::{
    GameState,
    jsDrawImage,
};

pub fn draw_ui(state: &GameState) -> Option<()> {
    let ci = state.entities[0].combat_info.as_ref()?;
    draw_status_bar(state, ci.health, ci.max_health, true);
    draw_status_bar(state, ci.cognition, ci.max_cognition, false);
    Some(())
}

fn draw_status_bar(state: &GameState, value: u32, max_value: u32, is_health: bool) {
    // for now lets say canvas_width = 100 hp
    let tile_pix: f32 = state.camera.tile_pix as f32;
    let bar_width_max: f32 = max_value as f32 / 100.0 * state.camera.canvas_width as f32;
    let bar_width_cur: f32 = value as f32 / 100.0 * state.camera.canvas_width as f32;

    let x_pos = 10.0;
    let y_pos = 10.0 + if is_health { 0.0 } else { tile_pix/2.0 };
    let fill = if is_health { "health_fill" } else { "cog_fill" };

    draw_as_3patch(state, "status_bg",
                   11, 0, 0, 74, 40, x_pos, y_pos, bar_width_max, tile_pix/2.0);

    jsDrawImage(&state.ctx, fill,
                0, 0, 74, 40,
                x_pos, y_pos, bar_width_cur, tile_pix/2.0, false);

    draw_as_3patch(state, "status_cover",
                   11, 0, 0, 74, 40, x_pos, y_pos, bar_width_max, tile_pix/2.0);
}

fn draw_as_3patch(state: &GameState, name: &str, edge: u32,
                  sx: u32, sy: u32, sw: u32, sh: u32,
                  x: f32, y: f32, w: f32, h: f32) {
    jsDrawImage(&state.ctx, name,
                sx, sy, edge, sh,
                x, y, edge as f32, h, false);
    jsDrawImage(&state.ctx, name,
                sx+edge, sy, sw-2*edge, sh,
                x+edge as f32, y, w-2.0*edge as f32, h, false);
    jsDrawImage(&state.ctx, name,
                sx+sw-edge, sy, edge, sh,
                x+w-edge as f32, y, edge as f32, h, false);
}

//status bars are 74 x 40

/*
Experience bar

float imHeight = Constants.HEALTH_BAR_SIZE / 6;
experienceImage = new Image(resourceManager.findRegion("experience_bar"));
experienceImage.setBounds(0, height - imHeight*.75f, width*experiencePercent, imHeight/2);
*/

