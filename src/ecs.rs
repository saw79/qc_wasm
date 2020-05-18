
// ------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct LogicalPos {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug)]
pub struct RenderInfo {
    pub x: f32,
    pub y: f32,
    pub sheet_name: &'static str,
    pub sheet_x: u32,
    pub sheet_y: u32,
    pub sheet_w: u32,
    pub sheet_h: u32,
}

#[derive(Debug, Clone)]
pub enum Action {
    Move(u32, u32),
    Attack,
}

#[derive(Debug)]
pub struct ActionQueue {
    pub current: Option<Action>,
    pub queue: Vec<Action>,
}

/*
#[derive(Debug)]
pub struct Target {
    pub x: u32,
    pub y: u32,
}
*/

// ------------------------------------------------------------

pub struct Entity {
    pub logical_pos: Option<LogicalPos>,
    pub render_info: Option<RenderInfo>,
    pub action_queue: Option<ActionQueue>,
    //pub target: Option<Target>,
}

