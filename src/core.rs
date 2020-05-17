pub const MOVE_SPEED: f32 = 10.0; // tiles/s

const Y_TILES: u32 = 20;

#[derive(Clone)]
pub enum TileType {
    FLOOR,
    WALL,
    DOORCLOSED,
    DOOROPEN,
}

pub struct TileGrid {
    pub width: usize,
    pub height: usize,
    tiles: Vec<Vec<TileType>>,
}

impl TileGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let mut tiles = vec![vec![TileType::FLOOR; width]; height];

        for x in 0..width {
            tiles[0][x] = TileType::WALL;
            tiles[height-1][x] = TileType::WALL;
        }
        for y in 0..height {
            tiles[y][0] = TileType::WALL;
            tiles[y][width-1] = TileType::WALL;
        }

        tiles[1][1] = TileType::DOOROPEN;
        tiles[1][2] = TileType::DOORCLOSED;

        TileGrid {
            width: width,
            height: height,
            tiles: tiles,
        }
    }

    pub fn at(&self, x: usize, y: usize) -> &TileType {
        &self.tiles[y][x]
    }
}

pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub tile_pix: u32,
}

impl Camera {
    pub fn new(grid_w: u32, grid_h: u32, canvas_w: u32, canvas_h: u32) -> Self {
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

