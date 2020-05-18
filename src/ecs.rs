
// ------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct LogicalPos {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug)]
pub struct RenderFrame {
    pub sheet_name: &'static str,
    pub sheet_x: u32,
    pub sheet_y: u32,
    pub sheet_w: u32,
    pub sheet_h: u32,
}

#[derive(Debug)]
pub struct RenderInfo {
    pub x: f32,
    pub y: f32,
    pub active: bool,
    pub time: f32,
    pub frame_duration: f32,
    pub curr_frame: usize,
    pub frames: Vec<RenderFrame>,
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

#[derive(Debug)]
pub struct CombatInfo {
    pub health: u32,
    pub max_health: u32,
    pub cognition: u32,
    pub max_cognition: u32,
    pub damage: u32,
    pub absorption: u32,
    pub dodge: u32,
}

// ------------------------------------------------------------

pub struct Entity {
    pub name: &'static str,
    pub logical_pos: Option<LogicalPos>,
    pub render_info: Option<RenderInfo>,
    pub action_queue: Option<ActionQueue>,
    pub combat_info: Option<CombatInfo>,
}

