use std::f32;

use constants::{PLAYER_VISION, ENEMY_VISION};
use bresenham::get_line;

use debug::log;

const Y_TILES: i32 = 20;

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TileType {
    FLOOR,
    WALL,
    DOORCLOSED,
    DOOROPEN,
}

#[derive(Clone, PartialEq)]
pub enum Visibility {
    UNSEEN,
    SEEN,
    VISIBLE,
}

pub struct TileGrid {
    pub width: usize,
    pub height: usize,
    tiles: Vec<Vec<TileType>>,
    visibility: Vec<Vec<Visibility>>,
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

        for y in 2..height-2 {
            tiles[y][20] = TileType::WALL;
        }

        TileGrid {
            width: width,
            height: height,
            tiles: tiles,
            visibility: vec![vec![Visibility::UNSEEN; width]; height],
        }
    }

    pub fn at(&self, x: i32, y: i32) -> &TileType {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            &TileType::WALL
        } else {
            &self.tiles[y as usize][x as usize]
        }
    }

    pub fn get_visibility(&self, x: i32, y: i32) -> &Visibility {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            &Visibility::UNSEEN
        } else {
            &self.visibility[y as usize][x as usize]
        }
    }

    pub fn set_visibility(&mut self, x: i32, y: i32, visibility: Visibility) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            self.visibility[y as usize][x as usize] = visibility;
        }
    }

    pub fn update_visibility(&mut self, x0: i32, y0: i32) {
        for x in 0..self.width {
            for y in 0..self.height {
                let (x, y) = (x as i32, y as i32);

                // reset all VISIBLE (to be filled in)
                if self.get_visibility(x, y) == &Visibility::VISIBLE {
                    self.set_visibility(x, y, Visibility::SEEN);
                }

                // fill in potential VISIBLE's
                if (x - x0).abs() <= PLAYER_VISION && (y - y0).abs() <= PLAYER_VISION {
                    let line = get_line(x0, y0, x, y);
                    for (xi, yi) in line {
                        self.set_visibility(xi, yi, Visibility::VISIBLE);
                        if self.at(xi, yi) == &TileType::WALL || self.at(xi, yi) == &TileType::DOORCLOSED {
                            break;
                        }
                    }
                }
            }
        }
    }

    pub fn visibility_from_to(&self, x0: i32, y0: i32, x1: i32, y1: i32, dir: &Direction) -> bool {
        // 1. check direction and radius
        let dist = (((x1 - x0).pow(2) + (y1 - y0).pow(2)) as f32).sqrt();
        if dist > ENEMY_VISION as f32 {
            return false;
        }

        let pl_angle = ((y1 - y0) as f32).atan2((x1 - x0) as f32);
        let look_angle = match dir {
            &Direction::Right => 0.0,
            &Direction::Down => f32::consts::PI/2.0,
            &Direction::Left => f32::consts::PI*2.0/2.0,
            &Direction::Up => f32::consts::PI*3.0/2.0,
        };
        if 1.0 - (pl_angle-look_angle).cos() > 0.708 { // cos(pi/4)
            return false;
        }

        // 2. check tilegrid obstruction
        let line = get_line(x0, y0, x1, y1);
        for (xi, yi) in line {
            if self.at(xi, yi) == &TileType::WALL || self.at(xi, yi) == &TileType::DOORCLOSED {
                return false;
            }
        }
        
        true
    }
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
    pub total_time: f32,
    pub curr_time: f32,
    pub x: f32,
    pub y: f32,
}

