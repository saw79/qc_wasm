
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
    Wait,
    Move(u32, u32),
    Attack(usize),
}

#[derive(Debug)]
pub struct ActionQueue {
    pub current: Option<Action>,
    pub queue: Vec<Action>,
}

#[derive(Debug)]
pub struct EntityTarget {
    pub id: usize,
}

#[derive(Debug)]
pub struct CombatInfo {
    pub health: i32,
    pub max_health: i32,
    pub cognition: i32,
    pub max_cognition: i32,
    pub damage: i32,
    pub absorption: i32,
    pub dodge: i32,
    pub current_attack: Option<usize>,
}

// ------------------------------------------------------------

pub struct Entity {
    pub name: &'static str,
    pub logical_pos: Option<LogicalPos>,
    pub render_info: Option<RenderInfo>,
    pub action_queue: Option<ActionQueue>,
    pub entity_target: Option<EntityTarget>,
    pub combat_info: Option<CombatInfo>,
}

