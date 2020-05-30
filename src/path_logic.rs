use pathfinding::prelude::{absdiff, astar};

use tile_grid::{TileType, TileGrid};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos(pub i32, pub i32);

impl Pos {
    fn distance(&self, other: &Pos) -> u32 {
        (absdiff(self.0, other.0) + absdiff(self.1, other.1)) as u32
    }

    fn neighbors(&self, tile_grid: &TileGrid) -> Vec<(Pos, u32)> {
        let &Pos(x, y) = self;
        let mut nbs = vec![];
        for dx in -1..2 {
            for dy in -1..2 {
                if dx == 0 && dy == 0 {
                    continue
                }

                let nx = x + dx;
                let ny = y + dy;
                let mut valid = false;
                if nx >= 0 && nx < tile_grid.width as i32 && ny >= 0 && ny < tile_grid.height as i32 {
                    match tile_grid.at(nx, ny) {
                        &TileType::EMPTY      => {},
                        &TileType::FLOOR      => valid = true,
                        &TileType::WALL       => {},
                        &TileType::DOORCLOSED => valid = true,
                        &TileType::DOOROPEN   => valid = true,
                    };
                }

                if valid {
                    let pp = Pos(nx, ny);
                    let dist = self.distance(&pp);
                    nbs.push((pp, dist));
                }
            }
        }

        nbs
    }
}

pub fn get_path(
    x0: i32, y0: i32,
    x1: i32, y1: i32,
    tile_grid: &TileGrid)
    -> Option<(Vec<Pos>, u32)> {
    //

    let start = Pos(x0, y0);
    let goal = Pos(x1, y1);
    astar(&start, |p| p.neighbors(tile_grid), |p| p.distance(&goal), |p| *p == goal)
}

