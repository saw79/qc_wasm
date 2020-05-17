use pathfinding::prelude::{absdiff, astar};

use debug::log;

use core::{TileType, TileGrid};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos(pub u32, pub u32);

impl Pos {
    fn distance(&self, other: &Pos) -> u32 {
        //(absdiff(self.0, other.0) + absdiff(self.1, other.1)) as u32
        absdiff(self.0, other.0) + absdiff(self.1, other.1)
    }

    fn neighbors(&self, tile_grid: &TileGrid) -> Vec<(Pos, u32)> {
        let &Pos(x, y) = self;
        let mut nbs = vec![];
        for dx in -1..2 {
            for dy in -1..2 {
                if dx == 0 && dy == 0 {
                    continue
                }

                let nx: i32 = x as i32 + dx;
                let ny: i32 = y as i32 + dy;
                if nx >= 0 && nx < tile_grid.width as i32 &&
                   ny >= 0 && ny < tile_grid.height as i32 &&
                   *tile_grid.at(nx as usize, ny as usize) != TileType::WALL {
                    nbs.push(Pos(nx as u32, ny as u32));
                }
            }
        }

        nbs.into_iter().map(|p| (p, 1)).collect()
    }
}

pub fn get_path(
    x0: u32, y0: u32,
    x1: u32, y1: u32,
    tile_grid: &TileGrid)
    -> Option<(Vec<Pos>, u32)> {
    //

    let start = Pos(x0, y0);
    let goal = Pos(x1, y1);
    astar(&start, |p| p.neighbors(tile_grid), |p| p.distance(&goal), |p| *p == goal)
}

/*
pub fn test() {
    test_full();
}

fn test_full() {
    let start = Pos(1, 1);
    let goal = Pos(4, 6);
    let result: Option<(Vec<Pos>, u32)> =
        astar(&start, |p| p.neighbors(), |p| p.distance(&goal)/3, |p| *p == goal);

    match result {
        Some((path, cost)) => {
            console_log!("Success :)");
            console_log!("{:?}", path);
            console_log!("{}", cost);
        },
        None => console_log!("Failure :("),
    };
}
*/

