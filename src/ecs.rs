use std::collections::HashMap;

// ------------------------------------------------------------

#[derive(Clone)]
pub struct CTilePos {
    pub x: u32,
    pub y: u32,
}

pub struct CDrawPos {
    pub x: f32,
    pub y: f32,
}

pub struct CTarget {
    pub x: u32,
    pub y: u32,
}

// ------------------------------------------------------------

pub struct Entity {
    pub tile_pos: Option<CTilePos>,
    pub draw_pos: Option<CDrawPos>,
    pub target: Option<CTarget>,
}

