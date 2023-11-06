use super::Layer;
use crate::dungeon::enums::{CornerType, DoorType, TileType, WallType};
use rand::{rngs::ThreadRng, Rng};
use std::cmp::{max, min};
use std::fmt;

pub struct RoomLayer<const ROW: usize, const COLUMN: usize> {
    pub layer: Layer<TileType, ROW, COLUMN>,
    pub rooms: Vec<Room>,
}

impl<const ROW: usize, const COLUMN: usize> RoomLayer<ROW, COLUMN> {
    pub fn new(scale: f32, room_amount: usize) -> RoomLayer<ROW, COLUMN> {
        let mut layer = Layer::new(TileType::Empthy, scale);
        let rooms = generate_rooms(ROW, COLUMN, room_amount);

        let mut rng = rand::thread_rng();

        let mut prev_i: i32 = 0;
        let mut prev_j: i32 = 0;

        for room in rooms.iter() {
            apply_room_to_map(&mut layer, &room);

            let (new_i, new_j) = room.center();
            if rng.gen_range(1..=2) == 1 {
                apply_row_tunnel(&mut layer, prev_i, new_i, prev_j);
                apply_column_tunnel(&mut layer, new_i, prev_j, new_j);
            } else {
                apply_column_tunnel(&mut layer, prev_i, prev_j, new_j);
                apply_row_tunnel(&mut layer, prev_i, new_i, new_j);
            }
            (prev_i, prev_j) = (new_i, new_j);
        }

        for (i, j, el) in layer.clone().windows_2x2() {
            match el {
                | [[TileType::Empthy, TileType::Empthy], [TileType::Empthy, TileType::Floor]] => {
                    layer[(i + 1, j + 1)] =
                        TileType::Wall(WallType::InternalCorner(CornerType::BottomLeft));
                }
                | [[TileType::Empthy, TileType::Empthy], [TileType::Floor, TileType::Empthy]] => {
                    layer[(i + 1, j)] =
                        TileType::Wall(WallType::InternalCorner(CornerType::BottomRight));
                }
                | [[TileType::Empthy, TileType::Floor], [TileType::Empthy, TileType::Empthy]] => {
                    layer[(i, j + 1)] =
                        TileType::Wall(WallType::InternalCorner(CornerType::TopLeft));
                }
                | [[TileType::Floor, TileType::Empthy], [TileType::Empthy, TileType::Empthy]] => {
                    layer[(i, j)] = TileType::Wall(WallType::InternalCorner(CornerType::TopRight));
                }
                | _ => {}
            }
        }

        for (i, j, el) in layer.clone().windows_1x3() {
            match el {
                | [[TileType::Empthy, TileType::Floor, TileType::Floor]] => {
                    layer[(i, j + 1)] = TileType::Wall(WallType::Left)
                }
                | [[TileType::Floor, TileType::Floor, TileType::Empthy]] => {
                    layer[(i, j + 1)] = TileType::Wall(WallType::Right)
                }
                | _ => {}
            }
        }

        for (i, j, el) in layer.clone().windows_3x1() {
            match el {
                | [[TileType::Empthy], [TileType::Floor], [TileType::Floor]] => {
                    layer[(i + 1, j)] = TileType::Wall(WallType::Bottom)
                }
                | [[TileType::Floor], [TileType::Floor], [TileType::Empthy]] => {
                    layer[(i + 1, j)] = TileType::Wall(WallType::Top)
                }
                | _ => {}
            }
        }

        for (i, j, el) in layer.clone().windows_2x1() {
            match el {
                | [[TileType::Floor], [TileType::Path]] => {
                    layer[(i, j)] = TileType::Door(DoorType::Right);
                }
                | [[TileType::Path], [TileType::Floor]] => {
                    layer[(i + 1, j)] = TileType::Door(DoorType::Left);
                }
                | [[TileType::Empthy], [TileType::Floor | TileType::Path]] => {
                    //layer[(i, j)] = TileType::Wall(WallType::Bottom)
                }
                | [[TileType::Floor | TileType::Path], [TileType::Empthy]] => {
                    //layer[(i + 1, j)] = TileType::Wall(WallType::Top)
                }
                | _ => {}
            }
        }

        for (i, j, el) in layer.clone().windows_1x2() {
            match el {
                | [[TileType::Floor, TileType::Path]] => {
                    layer[(i, j)] = TileType::Door(DoorType::Bottom)
                }
                | [[TileType::Path, TileType::Floor]] => {
                    layer[(i, j + 1)] = TileType::Door(DoorType::Top)
                }
                | [[TileType::Floor | TileType::Path, TileType::Empthy]] => {
                    //layer[(i, j)] = TileType::Wall(WallType::Right)
                }
                | [[TileType::Empthy, TileType::Floor | TileType::Path]] => {
                    //layer[(i, j)] = TileType::Wall(WallType::Left)
                }
                | _ => {}
            }
        }

        RoomLayer { layer, rooms }
    }
}

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
        let diff_i = (self.i - other.i).abs() * 2 - 3;
        let diff_j = (self.j - other.j).abs() * 2 - 3;
        let rooms_row = self.row + other.row;
        let rooms_column = self.column + other.column;

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

pub fn apply_room_to_map<const ROW: usize, const COLUMN: usize>(
    layer: &mut Layer<TileType, ROW, COLUMN>,
    room: &Room,
) {
    for i in room.i..=room.i + room.row {
        for j in room.j..=room.j + room.column {
            layer[(i as usize, j as usize)] = TileType::Floor;
        }
    }
}

pub fn apply_row_tunnel<const ROW: usize, const COLUMN: usize>(
    layer: &mut Layer<TileType, ROW, COLUMN>,
    i1: i32,
    i2: i32,
    j: i32,
) {
    for i in min(i1, i2)..=max(i1, i2) {
        if layer[(i as usize, j as usize)] == TileType::Empthy {
            layer[(i as usize, j as usize)] = TileType::Path;
        }
    }
}

pub fn apply_column_tunnel<const ROW: usize, const COLUMN: usize>(
    layer: &mut Layer<TileType, ROW, COLUMN>,
    i: i32,
    j1: i32,
    j2: i32,
) {
    for j in min(j1, j2)..=max(j1, j2) {
        if layer[(i as usize, j as usize)] == TileType::Empthy {
            layer[(i as usize, j as usize)] = TileType::Path;
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
