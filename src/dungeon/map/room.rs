use super::{Layer, TileType};
use rand::{rngs::ThreadRng, Rng};
use std::cmp::{max, min};
use std::fmt;

pub struct Room {
    pub width_coord: i32,
    pub length_coord: i32,
    pub width: i32,
    pub length: i32,
}

impl Room {
    pub fn new(width_coord: i32, length_coord: i32, width: i32, length: i32) -> Room {
        Room {
            width_coord,
            length_coord,
            width,
            length,
        }
    }

    pub fn intersect(&self, other: &Room) -> bool {
        let diff_width_coord = (self.width_coord - other.width_coord).abs();
        let diff_length_coord = (self.length_coord - other.length_coord).abs();
        let max_width = max(self.width, other.width);
        let max_length = max(self.length, other.length);

        diff_width_coord < max_width && diff_length_coord < max_length
    }

    pub fn center(&self) -> (i32, i32) {
        (
            (self.width_coord + self.width / 2),
            (self.length_coord + self.length / 2),
        )
    }
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Room::new({}, {}, {}, {})",
            self.width_coord, self.length_coord, self.width, self.length
        )
    }
}

pub fn apply_room_to_map(map: &mut Layer<TileType>, room: &Room) {
    for width_coord in room.width_coord..=room.width_coord + room.width {
        for length_coord in room.length_coord..=room.length_coord + room.length {
            map.set(width_coord as usize, length_coord as usize, TileType::Floor);
        }
    }
}

pub fn apply_width_tunnel(
    map: &mut Layer<TileType>,
    width_coord_1: i32,
    width_coord_2: i32,
    length_coord: i32,
) {
    for width_coord in min(width_coord_1, width_coord_2)..=max(width_coord_1, width_coord_2) {
        map.set(width_coord as usize, length_coord as usize, TileType::Floor);
    }
}

pub fn apply_length_tunnel(
    map: &mut Layer<TileType>,
    width_coord: i32,
    length_coord_1: i32,
    length_coord_2: i32,
) {
    for length_coord in min(length_coord_1, length_coord_2)..=max(length_coord_1, length_coord_2) {
        map.set(width_coord as usize, length_coord as usize, TileType::Floor);
    }
}

fn get_random_room(
    rng: &mut ThreadRng,
    max_width_coord: i32,
    max_length_coord: i32,
    min_size: i32,
    max_size: i32,
) -> Room {
    let width = rng.gen_range(min_size..=max_size);
    let length = rng.gen_range(min_size..=max_size);
    let width_coord = rng.gen_range(1..(max_width_coord - width)) - 1;
    let length_coord = rng.gen_range(1..(max_length_coord - length)) - 1;
    Room::new(width_coord, length_coord, width, length)
}

pub fn generate_rooms(layer_width: usize, layer_length: usize, room_amount: usize) -> Vec<Room> {
    let mut rooms: Vec<Room> = Vec::new();
    const MIN_SIZE: i32 = 4;
    const MAX_SIZE: i32 = 6;

    let mut rng = rand::thread_rng();
    for _ in 0..room_amount {
        let new_room = get_random_room(
            &mut rng,
            layer_width as i32,
            layer_length as i32,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersections() -> Result<(), String> {
        let square_room_3 = |width_coord, length_coord| Room::new(width_coord, length_coord, 3, 3);

        [
            (square_room_3(0, 0), square_room_3(2, 2), true),
            (square_room_3(0, 0), square_room_3(3, 3), false),
            (square_room_3(0, 0), square_room_3(2, 0), true),
            (square_room_3(0, 0), square_room_3(3, 0), false),
            (square_room_3(0, 0), square_room_3(0, 2), true),
            (square_room_3(0, 0), square_room_3(0, 3), false),
        ]
        .iter()
        .try_for_each(|(room1, room2, expected)| {
            if room1.intersect(room2) == *expected {
                Ok(())
            } else {
                Err(format!(
                    "{}.intersect({}) result: {}, expected: {}",
                    room1,
                    room2,
                    room1.intersect(room2),
                    expected
                ))
            }
        })?;
        Ok(())
    }
}
