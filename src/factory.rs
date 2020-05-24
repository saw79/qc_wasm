use constants::ENEMY_VISION;
use core::Direction;
use ecs::*;

pub fn create_player(x: i32, y: i32) -> Entity {
    Entity {
        name: "player_none",
        logical_pos: Some(LogicalPos { x: x, y: y }),
        vision_info: None,
        render_info: Some(RenderInfo {
            x: x as f32,
            y: y as f32,
            active: false,
            time: 0.0,
            frame_duration: 0.1,
            curr_frame: 0,
            frames: get_walk_anim("player_none", &Direction::Down),
        }),
        action_queue: Some(ActionQueue {
            current: None,
            queue: vec![],
        }),
        entity_target: None,
        combat_info: Some(CombatInfo {
            health: 10,
            max_health: 10,
            cognition: 10,
            max_cognition: 10,
            damage: 1,
            absorption: 0,
            dodge: 0,
            current_attack: None,
        }),
    }
}

pub fn create_enemy(x: i32, y: i32, name: &'static str) -> Entity {
    let dir = Direction::Down;

    Entity {
        name: name,
        logical_pos: Some(LogicalPos { x: x, y: y }),
        vision_info: Some(VisionInfo {
            is_wedge: true,
            radius: ENEMY_VISION,
            dir: dir.clone(),
            alert_state: AlertState::PATROL,
            last_location: (x, y),
        }),
        render_info: Some(RenderInfo {
            x: x as f32,
            y: y as f32,
            active: false,
            time: 0.0,
            frame_duration: 0.1,
            curr_frame: 0,
            frames: get_walk_anim(name, &dir),
        }),
        action_queue: Some(ActionQueue {
            current: None,
            queue: vec![],
        }),
        entity_target: None,
        combat_info: Some(CombatInfo {
            health: 10,
            max_health: 10,
            cognition: 10,
            max_cognition: 10,
            damage: 1,
            absorption: 0,
            dodge: 0,
            current_attack: None,
        }),
    }
}

pub fn create_orb(x: i32, y: i32, name: &'static str) -> Entity {
    Entity {
        name: name,
        logical_pos: Some(LogicalPos { x: x, y: y }),
        vision_info: None,
        render_info: Some(RenderInfo {
            x: x as f32,
            y: y as f32,
            active: true,
            time: 0.0,
            frame_duration: 0.1,
            curr_frame: 0,
            frames: get_orb_anim(name),
        }),
        action_queue: None,
        entity_target: None,
        combat_info: None,
    }
}

pub fn get_walk_anim(name: &'static str, dir: &Direction) -> Vec<RenderFrame> {
    let row = match dir {
        Direction::Up => 1,
        Direction::Down => 0,
        Direction::Left => 3,
        Direction::Right => 2,
    };

    let num_frames = 4;

    (0..num_frames).collect::<Vec<i32>>().iter().map(|c| RenderFrame {
        sheet_name: name,
        sheet_x: c*32,
        sheet_y: row*32,
        sheet_w: 32,
        sheet_h: 32,
    }).collect()
}

fn get_orb_anim(name: &'static str) -> Vec<RenderFrame> {
    vec![
        RenderFrame {
            sheet_name: name,
            sheet_x: 0,
            sheet_y: 0,
            sheet_w: 64,
            sheet_h: 64,
        },
        RenderFrame {
            sheet_name: name,
            sheet_x: 64,
            sheet_y: 0,
            sheet_w: 64,
            sheet_h: 64,
        },
        RenderFrame {
            sheet_name: name,
            sheet_x: 0,
            sheet_y: 64,
            sheet_w: 64,
            sheet_h: 64,
        },
        RenderFrame {
            sheet_name: name,
            sheet_x: 64,
            sheet_y: 64,
            sheet_w: 64,
            sheet_h: 64,
        },
        ]
}

