use core::Direction;

#[derive(Debug, Clone)]
pub struct LogicalPos {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, PartialEq)]
pub enum AlertState {
    PATROL,
    SEARCH,
    KILL,
}

pub struct VisionInfo {
    pub is_wedge: bool,
    pub radius: i32,
    pub dir: Direction,
    pub alert_state: AlertState,
    pub last_location: (i32, i32),
}

#[derive(Debug)]
pub struct RenderFrame {
    pub sheet_name: &'static str,
    pub sheet_x: i32,
    pub sheet_y: i32,
    pub sheet_w: i32,
    pub sheet_h: i32,
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
    Move(i32, i32),
    Attack(usize),
    Look(Direction),
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

#[derive(Debug, Clone)]
pub enum PickupAction {
    DIE,
    HEALTH_40P,
    COGNITION_40P,
}

pub struct PickupInfo {
    pub actions: Vec<PickupAction>,
}

// ------------------------------------------------------------

pub struct Entity {
    pub name: &'static str,
    pub dead: bool,
    pub logical_pos: Option<LogicalPos>,
    pub vision_info: Option<VisionInfo>,
    pub render_info: Option<RenderInfo>,
    pub action_queue: Option<ActionQueue>,
    pub entity_target: Option<EntityTarget>,
    pub combat_info: Option<CombatInfo>,
    pub pickup_info: Option<PickupInfo>,
}

