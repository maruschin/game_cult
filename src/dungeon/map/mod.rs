mod layer;


pub use layer::Layer;

use rand::Rng;
use std::cmp::{max, min};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn idx_xy(idx: usize) -> (i32, i32) {
    (
        (idx as i32) / (DUNGEON_WIDE as i32),
        (idx as i32) % (DUNGEON_WIDE as i32),
    )
}

pub struct Room {
    pub x: i32,
    pub y: i32,
    pub wide: i32,
    pub height: i32,
}

impl Room {
    pub fn new(x: i32, y: i32, wide: i32, height: i32) -> Room {
        Room {
            x: x,
            y: y,
            wide: wide,
            height: height,
        }
    }

    pub fn intersect(&self, other: &Room) -> bool {
        self.x <= other.x
            && self.x + self.wide >= other.x
            && self.y <= other.y
            && self.y + self.height >= other.y
    }

    pub fn center(&self) -> (i32, i32) {
        ((self.x + self.wide / 2), (self.y + self.height / 2))
    }
}

fn apply_room_to_map(map: &mut Layer<TileType>, room: &Room) {
    for y in room.y..=room.y + room.wide {
        for x in room.x..=room.x + room.height {
            map.set(x as usize, y as usize, TileType::Floor);
        }
    }
}

fn apply_horizontal_tunnel(map: &mut Layer<TileType>, x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
        map.set(x as usize, y as usize, TileType::Floor);
    }
}

fn apply_vertical_tunnel(map: &mut Layer<TileType>, y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        map.set(x as usize, y as usize, TileType::Floor);
    }
}

pub const DUNGEON_WIDE: usize = 20;
pub const DUNGEON_HEIGHT: usize = 10;

pub fn new_map() -> Layer<TileType> {
    let mut map: Layer<TileType> = Layer::new(TileType::Wall, DUNGEON_WIDE, DUNGEON_HEIGHT);

    let mut rooms: Vec<Room> = Vec::new();
    const MAX_ROOMS: i32 = 4;
    const MIN_SIZE: i32 = 2;
    const MAX_SIZE: i32 = 4;

    let mut rng = rand::thread_rng();

    for _ in 0..MAX_ROOMS {
        let wide = rng.gen_range(MIN_SIZE..=MAX_SIZE);
        let height = rng.gen_range(MIN_SIZE..=MAX_SIZE);
        let x = rng.gen_range(1..(DUNGEON_WIDE as i32 - wide)) - 1;
        let y = rng.gen_range(1..(DUNGEON_HEIGHT as i32 - height)) - 1;
        let new_room = Room::new(x, y, wide, height);
        let mut ok = true;
        for other_room in rooms.iter() {
            if new_room.intersect(other_room) {
                ok = false
            }
        }
        if ok {
            apply_room_to_map(&mut map, &new_room);

            if !rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                if rng.gen_range(1..=2) == 1 {
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
                    apply_vertical_tunnel(&mut map, prev_y, new_y, new_x);
                } else {
                    apply_vertical_tunnel(&mut map, prev_y, new_y, prev_x);
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, new_y);
                }
            }

            rooms.push(new_room);
        }
    }

    //let mut rng = rand::thread_rng();
    // rng.gen_range(1..6)
    map
}
