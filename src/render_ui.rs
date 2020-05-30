use crate::{
    GameState,
    jsDrawImage,
};

use user_interface::{ButtonType, ButtonState};

pub fn draw_ui(state: &GameState) -> Option<()> {
    let ci = state.entity_map.get(&0)?.combat_info.as_ref()?;
    draw_status_bar(state, ci.health, ci.max_health, true);
    draw_status_bar(state, ci.cognition, ci.max_cognition, false);
    draw_main_buttons(state);
    Some(())
}

fn draw_status_bar(state: &GameState, value: i32, max_value: i32, is_health: bool) {
    // for now lets say canvas_width = 100 hp
    let tile_pix = state.camera.tile_pix;
    let max_width: f32 = (state.camera.canvas_width - 10*2) as f32;
    let bar_width_max: i32 = (max_value as f32 / 100.0 * max_width) as i32;
    let bar_width_cur: i32 = (value as f32 / 100.0 * max_width) as i32;

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

fn draw_as_3patch(state: &GameState, name: &str, edge: i32,
                  sx: i32, sy: i32, sw: i32, sh: i32,
                  x: i32, y: i32, w: i32, h: i32) {
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

/*
assets["btn_small_up"] = loadImage("assets/UIImages/button_small_up.png");
assets["btn_small_down"] = loadImage("assets/UIImages/button_small_down.png");
assets["btn_small_checked"] = loadImage("assets/UIImages/button_small_checked.png");
*/
fn draw_main_buttons(state: &GameState) {
    for button in state.ui.buttons.values() {
        match button.state {
            ButtonState::UP => jsDrawImage(&state.ctx, "btn_small_up", 0, 0, 128, 128,
                                           button.x, button.y, button.width, button.height),
            ButtonState::DOWN => jsDrawImage(&state.ctx, "btn_small_down", 0, 0, 128, 128,
                                             button.x, button.y, button.width, button.height),
            ButtonState::CHECKED => jsDrawImage(&state.ctx, "btn_small_checked", 0, 0, 128, 128,
                                                button.x, button.y, button.width, button.height),
        };
        match &button.skin_img {
            Some(si) => jsDrawImage(&state.ctx, &si, 0, 0, 64, 64,
                                    button.x+button.width/4, button.y+button.height/4,
                                    button.width/2, button.height/2),
            None => {},
        };
    }
}

