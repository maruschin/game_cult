use super::room::{apply_column_tunnel, apply_room_to_map, apply_row_tunnel, generate_rooms, Room};
use super::{Layer, TileType};

use rand::Rng;
pub struct RoomLayer {
    pub layer: Layer<TileType>,
    pub rooms: Vec<Room>,
}

impl RoomLayer {
    pub fn new(row: usize, column: usize, scale: f32, room_amount: usize) -> RoomLayer {
        let mut layer = Layer::new(TileType::Wall, row, column, scale);
        let rooms = generate_rooms(layer.row, layer.column, room_amount);

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

        RoomLayer { layer, rooms }
    }
}
