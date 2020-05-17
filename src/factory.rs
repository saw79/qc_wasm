use ecs::*;

pub fn create_player(x: u32, y: u32) -> Entity {
    Entity {
        logical_pos: Some(LogicalPos { x: x, y: y }),
        render_info: Some(RenderInfo {
            x: x as f32,
            y: y as f32,
            sheet_name: "player_none",
            sheet_x: 0,
            sheet_y: 0,
            sheet_w: 32,
            sheet_h: 32,
        }),
        action_queue: None,
        target: None,
    }
}

pub fn create_target(x: u32, y: u32) -> Entity {
    Entity {
        logical_pos: Some(LogicalPos { x: x, y: y }),
        render_info: Some(RenderInfo {
            x: x as f32,
            y: y as f32,
            sheet_name: "target",
            sheet_x: 0,
            sheet_y: 0,
            sheet_w: 64,
            sheet_h: 64,
        }),
        action_queue: None,
        target: None,
    }
}

