use ecs::*;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn create_player(x: u32, y: u32) -> Entity {
    Entity {
        logical_pos: Some(LogicalPos { x: x, y: y }),
        render_info: Some(RenderInfo {
            x: x as f32,
            y: y as f32,
            active: false,
            time: 0.0,
            frame_duration: 0.1,
            curr_frame: 0,
            frames: get_walk_anim(&Direction::Down),
        }),
        action_queue: Some(ActionQueue {
            current: None,
            queue: vec![],
        }),
    }
}

pub fn create_target(x: u32, y: u32) -> Entity {
    Entity {
        logical_pos: Some(LogicalPos { x: x, y: y }),
        render_info: Some(RenderInfo {
            x: x as f32,
            y: y as f32,
            active: false,
            time: 0.0,
            frame_duration: 1.0,
            curr_frame: 0,
            frames: vec![
                RenderFrame {
                    sheet_name: "target",
                    sheet_x: 0,
                    sheet_y: 0,
                    sheet_w: 64,
                    sheet_h: 64,
                },
            ],
        }),
        action_queue: None,
    }
}

pub fn get_walk_anim(dir: &Direction) -> Vec<RenderFrame> {
    let row = match dir {
        Direction::Up => 1,
        Direction::Down => 0,
        Direction::Left => 3,
        Direction::Right => 2,
    };

    [0, 1, 2, 3].iter().map(|c| RenderFrame {
        sheet_name: "player_none",
        sheet_x: c*32,
        sheet_y: row*32,
        sheet_w: 32,
        sheet_h: 32,
    }).collect()
    /*vec![
        RenderFrame {
            sheet_name: "player_none",
            sheet_x: 0,
            sheet_y: row*32,
            sheet_w: 32,
            sheet_h: 32,
        },
        RenderFrame {
            sheet_name: "player_none",
            sheet_x: 32,
            sheet_y: row*32,
            sheet_w: 32,
            sheet_h: 32,
        },
        RenderFrame {
            sheet_name: "player_none",
            sheet_x: 64,
            sheet_y: row*32,
            sheet_w: 32,
            sheet_h: 32,
        },
        RenderFrame {
            sheet_name: "player_none",
            sheet_x: 96,
            sheet_y: row*32,
            sheet_w: 32,
            sheet_h: 32,
        },
    ]*/
}
