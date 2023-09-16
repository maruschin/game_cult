use super::{Layer, TileType};
use rand::{rngs::ThreadRng, Rng};
use std::cmp::{max, min};

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

fn get_random_room(
    rng: &mut ThreadRng,
    max_x: i32,
    max_y: i32,
    min_size: i32,
    max_size: i32,
) -> Room {
    let wide = rng.gen_range(min_size..=max_size);
    let height = rng.gen_range(min_size..=max_size);
    let x = rng.gen_range(1..(max_x - wide)) - 1;
    let y = rng.gen_range(1..(max_y - height)) - 1;
    Room::new(x, y, wide, height)
}

pub fn generate_rooms(layer_width: usize, layer_height: usize, room_amount: usize) -> Vec<Room> {
    let mut rooms: Vec<Room> = Vec::new();
    const MIN_SIZE: i32 = 4;
    const MAX_SIZE: i32 = 6;

    let mut rng = rand::thread_rng();
    for _ in 0..room_amount {
        let new_room = get_random_room(
            &mut rng,
            layer_width as i32,
            layer_height as i32,
            MIN_SIZE,
            MAX_SIZE,
        );

        if rooms
            .iter()
            .all(|other_room| !new_room.intersect(other_room))
        {
            rooms.push(new_room);
        }
    }
    return rooms;
}
