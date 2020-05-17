use ecs::*;

pub fn create_player(x: u32, y: u32) -> Entity {
    Entity {
        logical_pos: Some(LogicalPos { x: x, y: y }),
        visual_pos: Some(VisualPos { x: x as f32, y: y as f32 }),
        target: None,
    }
}

