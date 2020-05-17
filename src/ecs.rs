
// ------------------------------------------------------------

#[derive(Clone)]
pub struct LogicalPos {
    pub x: u32,
    pub y: u32,
}

pub struct VisualPos {
    pub x: f32,
    pub y: f32,
}

pub struct Target {
    pub x: u32,
    pub y: u32,
}

// ------------------------------------------------------------

pub struct Entity {
    pub logical_pos: Option<LogicalPos>,
    pub visual_pos: Option<VisualPos>,
    pub target: Option<Target>,
}

