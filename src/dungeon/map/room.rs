use super::{Layer, TileType};
use rand::{rngs::ThreadRng, Rng};
use std::cmp::{max, min};
use std::fmt;

pub struct Room {
    pub i: i32,
    pub j: i32,
    pub row: i32,
    pub column: i32,
}

impl Room {
    pub fn new(i: i32, j: i32, row: i32, column: i32) -> Room {
        Room { i, j, row, column }
    }

    pub fn intersect(&self, other: &Room) -> bool {
        let diff_i = (self.i - other.i).abs() * 2;
        let diff_j = (self.j - other.j).abs() * 2;
        let rooms_row = self.row + other.row + 2;
        let rooms_column = self.column + other.column + 2;

        diff_i <= rooms_row && diff_j <= rooms_column
    }

    pub fn center(&self) -> (i32, i32) {
        ((self.i + self.row / 2), (self.j + self.column / 2))
    }
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Room::new({}, {}, {}, {})",
            self.i, self.j, self.row, self.column
        )
    }
}

pub fn apply_room_to_map(layer: &mut Layer<TileType>, room: &Room) {
    for i in room.i..=room.i + room.row {
        for j in room.j..=room.j + room.column {
            layer.set(i as usize, j as usize, TileType::Floor);
        }
    }
}

pub fn apply_row_tunnel(layer: &mut Layer<TileType>, i1: i32, i2: i32, j: i32) {
    for i in min(i1, i2)..=max(i1, i2) {
        if layer.get(i as usize, j as usize) == TileType::Wall {
            layer.set(i as usize, j as usize, TileType::Path);
        }
    }
}

pub fn apply_column_tunnel(layer: &mut Layer<TileType>, i: i32, j1: i32, j2: i32) {
    for j in min(j1, j2)..=max(j1, j2) {
        if layer.get(i as usize, j as usize) == TileType::Wall {
            layer.set(i as usize, j as usize, TileType::Path);
        }
    }
}

fn get_random_room(
    rng: &mut ThreadRng,
    layer_row: i32,
    layer_column: i32,
    min_size: i32,
    max_size: i32,
) -> Room {
    let room_row = rng.gen_range(min_size..=max_size);
    let room_column = rng.gen_range(min_size..=max_size);
    let i = rng.gen_range(2..(layer_row - room_row)) - 1;
    let j = rng.gen_range(2..(layer_column - room_column)) - 1;
    Room::new(i, j, room_row, room_column)
}

pub fn generate_rooms(row: usize, column: usize, room_amount: usize) -> Vec<Room> {
    let mut rooms: Vec<Room> = Vec::new();
    const MIN_SIZE: i32 = 3;
    const MAX_SIZE: i32 = 4;

    let mut rng = rand::thread_rng();
    for _ in 0..room_amount * 4 {
        let new_room = get_random_room(&mut rng, row as i32, column as i32, MIN_SIZE, MAX_SIZE);

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
        let square_room_3 = |i, j| Room::new(i, j, 3, 3);

        [
            (square_room_3(0, 0), square_room_3(3, 3), true),
            (square_room_3(0, 0), square_room_3(4, 4), false),
            (square_room_3(0, 0), square_room_3(3, 0), true),
            (square_room_3(0, 0), square_room_3(4, 0), false),
            (square_room_3(0, 0), square_room_3(0, 3), true),
            (square_room_3(0, 0), square_room_3(0, 4), false),
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
