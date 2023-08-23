use std::cmp::{max, min};

use super::{Layer, TileType};

pub struct Room {
    pub x: i32,
    pub y: i32,
    pub wide: i32,
    pub height: i32,
}

impl Room {
    pub fn new(x: i32, y: i32, wide: i32, height: i32) -> Room {
        Room { x, y, wide, height }
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

pub fn apply_room_to_map(map: &mut Layer<TileType>, room: &Room) {
    for y in room.y..=room.y + room.wide {
        for x in room.x..=room.x + room.height {
            map.set(x as usize, y as usize, TileType::Floor);
        }
    }
}

pub fn apply_horizontal_tunnel(map: &mut Layer<TileType>, x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
        map.set(x as usize, y as usize, TileType::Floor);
    }
}

pub fn apply_vertical_tunnel(map: &mut Layer<TileType>, y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        map.set(x as usize, y as usize, TileType::Floor);
    }
}
