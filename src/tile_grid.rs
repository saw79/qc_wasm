use std::f32;

use constants::{PLAYER_VISION};
use core::Direction;
use util::rand_usize;
use bresenham::get_line;
use level_gen::{
    //gen_level_test,
    gen_level_bsp,
};

use debug::log;

#[derive(Debug, Clone, PartialEq)]
pub enum TileType {
    EMPTY,
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
        //let tiles = gen_level_test(width, height);
        let tiles = gen_level_bsp(width, height);

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

    /*pub fn update_visibility(&mut self, _x0: i32, _y0: i32) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.set_visibility(x as i32, y as i32, Visibility::VISIBLE);
            }
        }
    }*/

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

    pub fn visibility_from_to(&self, x0: i32, y0: i32, x1: i32, y1: i32,
                              max_dist: i32, dir_opt: Option<&Direction>) -> bool {
        // 0. special case of next to
        if x0 == x1 && (y1-y0).abs() <= 1 {
            let behind = match dir_opt {
                Some(&Direction::Down) if y1 == x0 - 1 => true,
                Some(&Direction::Up) if y1 == x0 + 1 => true,
                _ => false,
            };
            return !behind;
        }
        if y0 == y1 && (x1-x0).abs() <= 1 {
            let behind = match dir_opt {
                Some(&Direction::Right) if x1 == x0 - 1 => true,
                Some(&Direction::Left) if x1 == x0 + 1 => true,
                _ => false,
            };
            return !behind;
        }

        // 1. check direction and radius
        let dist = (((x1 - x0).pow(2) + (y1 - y0).pow(2)) as f32).sqrt();
        if dist > max_dist as f32 {
            return false;
        }

        if let Some(dir) = dir_opt {
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

    pub fn get_random_floor(&self) -> (i32, i32) {
        loop {
            let x = rand_usize(0, self.width);
            let y = rand_usize(0, self.width);
            if self.tiles[y][x] == TileType::FLOOR {
                return (x as i32, y as i32);
            }
        }
    }
}

