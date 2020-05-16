pub const Y_TILES: u32 = 20;
pub const MOVE_SPEED: f32 = 10.0; // tiles/s

pub enum TileType {
    FLOOR,
    WALL,
    DOORCLOSED,
    DOOROPEN,
}

pub struct TileGrid {
    pub width: u32,
    pub height: u32,
    tiles: Vec<Vec<TileType>>,
}

impl TileGrid {
    pub fn new(width: u32, height: u32) -> Self {
        TileGrid {
            width: width,
            height: height,
            tiles: vec![vec![]],
        }
    }
}

