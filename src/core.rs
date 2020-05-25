use constants;

const Y_TILES: i32 = 20;

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub canvas_width: i32,
    pub canvas_height: i32,
    pub tile_pix: i32,
}

impl Camera {
    pub fn new(grid_w: i32, grid_h: i32, canvas_w: i32, canvas_h: i32) -> Self {
        Camera {
            x: grid_w as f32 / 2.0,
            y: grid_h as f32 / 2.0,
            width: Y_TILES as f32 * (canvas_w as f32)/(canvas_h as f32),
            height: Y_TILES as f32,
            canvas_width: canvas_w,
            canvas_height: canvas_h,
            tile_pix: canvas_h / Y_TILES,
        }
    }
}

pub struct FloatingText {
    pub text: String,
    pub style: String,
    pub total_time: f32,
    pub curr_time: f32,
    pub x: f32,
    pub y: f32,
}

impl FloatingText {
    pub fn new(text: String, style: String, delay: f32, x: f32, y: f32) -> Self {
        FloatingText {
            text: text,
            style: style,
            total_time: constants::FLOATING_TEXT_TIME,
            curr_time: 0.0-delay,
            x: x,
            y: y + delay*constants::FLOATING_TEXT_SPEED,
        }
    }
}

