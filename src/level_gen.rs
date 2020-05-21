use tile_grid::TileType;
use util::rand_usize;

use debug::log;

type TileArr = Vec<Vec<TileType>>;

/*
pub fn gen_level_test(width: usize, height: usize) -> TileArr {
    let mut tiles = init_exterior(width, height);

    tiles[1][1] = TileType::DOOROPEN;
    tiles[1][2] = TileType::DOORCLOSED;

    for y in 2..height-2 {
        tiles[y][20] = TileType::WALL;
    }

    tiles
}
*/

fn init_exterior(width: usize, height: usize, fill: TileType) -> TileArr {
    let mut tiles = vec![vec![fill; width]; height];

    for x in 0..width {
        tiles[0][x] = TileType::WALL;
        tiles[height-1][x] = TileType::WALL;
    }
    for y in 0..height {
        tiles[y][0] = TileType::WALL;
        tiles[y][width-1] = TileType::WALL;
    }

    tiles
}

const MIN_ROOM_SIZE: usize = 8;

// includes boundary!!!
#[derive(Debug)]
struct Room {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

pub fn gen_level_bsp(width: usize, height: usize) -> TileArr {
    let mut tiles = init_exterior(width, height, TileType::WALL);

    let mut rooms = split_room(Room { x: 0, y: 0, width: width, height: height });

    for room in rooms.iter_mut() {
        shrink_room(room);
    }

    for room in &rooms {
        fill_room(&mut tiles, room);
    }

    for i0 in 0..rooms.len()-1 {
        let i1 = i0 + 1;
        connect_rooms(&mut tiles, &rooms[i0], &rooms[i1]);
    }

    tiles
}

fn split_room(room: Room) -> Vec<Room> {
    if room.width > room.height {
        split_room_hor(room)
    } else {
        split_room_vert(room)
    }
}

fn split_room_hor(room: Room) -> Vec<Room> {
    if room.width < 2*MIN_ROOM_SIZE - 1 {
        return vec![room];
    }

    let min_idx = room.x + MIN_ROOM_SIZE - 1;
    let max_idx = room.x + room.width - MIN_ROOM_SIZE;
    let split_idx = rand_usize(min_idx, max_idx + 1);

    let room0 = Room {
        x: room.x, y: room.y,
        width: split_idx - room.x + 1, height: room.height,
    };
    let room1 = Room {
        x: split_idx, y: room.y,
        width: room.x + room.width - split_idx, height: room.height,
    };

    let mut v = split_room(room0);
    v.append(&mut split_room(room1));
    v
}

fn split_room_vert(room: Room) -> Vec<Room> {
    if room.height < 2*MIN_ROOM_SIZE - 1 {
        return vec![room];
    }

    let min_idx = room.y + MIN_ROOM_SIZE - 1;
    let max_idx = room.y + room.height - MIN_ROOM_SIZE;
    let split_idx = rand_usize(min_idx, max_idx + 1);

    let room0 = Room {
        x: room.x, y: room.y,
        width: room.width, height: split_idx - room.y + 1,
    };
    let room1 = Room {
        x: room.x, y: split_idx,
        width: room.width, height: room.y + room.height - split_idx,
    };

    let mut v = split_room(room0);
    v.append(&mut split_room(room1));
    v
}

fn shrink_room(room: &mut Room) {
    let min_shrink = 2;
    let max_shrink = room.width/2 - 1;
    let width_shrink = rand_usize(min_shrink, max_shrink + 1);
    let x_off = rand_usize(0, width_shrink);
    room.x += x_off;
    room.width -= width_shrink;

    let min_shrink = 2;
    let max_shrink = room.height/2 - 1;
    let height_shrink = rand_usize(min_shrink, max_shrink + 1);
    let y_off = rand_usize(0, height_shrink);
    room.y += y_off;
    room.height -= height_shrink;
}

fn fill_room(tiles: &mut TileArr, room: &Room) {
    for x in room.x+1..room.x+room.width-1 {
        for y in room.y+1..room.y+room.height-1 {
            tiles[y][x] = TileType::FLOOR;
        }
    }
}

fn connect_rooms(tiles: &mut TileArr, room0: &Room, room1: &Room) {
    let left0 = room0.x + 1;
    let right0 = room0.x + room0.width - 2;
    let top0 = room0.y + 1;
    let bot0 = room0.y + room0.height - 2;

    let left1 = room1.x + 1;
    let right1 = room1.x + room1.width - 2;
    let top1 = room1.y + 1;
    let bot1 = room1.y + room1.height - 2;

    if left0 >= left1 && left0 <= right1 {
        connect_rooms_vert(tiles, room0, room1);
    }
    else if right0 >= left1 && right0 <= right1 {
        connect_rooms_vert(tiles, room0, room1);
    }
    else if top0 >= top1 && top0 <= bot1 {
        connect_rooms_hor(tiles, room0, room1);
    }
    else if bot0 >= top1 && bot0 <= bot1 {
        connect_rooms_hor(tiles, room0, room1);
    }
    else {
        // make fake 1-tile room with room0's x and room1's y
        let room_tmp = Room {
            x: rand_usize(room0.x+1, room0.x+room0.width-1) - 1,
            y: rand_usize(room1.y+1, room1.y+room1.height-1) - 1,
            width: 3,
            height: 3,
        };
        connect_rooms_vert(tiles, room0, &room_tmp);
        connect_rooms_hor(tiles, room1, &room_tmp);
    }
}

fn connect_rooms_hor(tiles: &mut TileArr, room0: &Room, room1: &Room) {
    let mut xs = vec![room0.x+1, room0.x+room0.width-2, room1.x+1, room1.x+room1.width-2];
    let mut ys = vec![room0.y+1, room0.y+room0.height-2, room1.y+1, room1.y+room1.height-2];
    xs.sort();
    ys.sort();

    let hall_y = rand_usize(ys[1], ys[2] + 1);

    for x in xs[1]..xs[2]+1 {
        tiles[hall_y][x] = TileType::FLOOR;
    }
}

fn connect_rooms_vert(tiles: &mut TileArr, room0: &Room, room1: &Room) {
    let mut xs = vec![room0.x+1, room0.x+room0.width-2, room1.x+1, room1.x+room1.width-2];
    let mut ys = vec![room0.y+1, room0.y+room0.height-2, room1.y+1, room1.y+room1.height-2];
    xs.sort();
    ys.sort();

    let hall_x = rand_usize(xs[1], xs[2] + 1);

    for y in ys[1]..ys[2]+1 {
        tiles[y][hall_x] = TileType::FLOOR;
    }
}

