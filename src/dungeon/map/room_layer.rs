use super::room::{
    apply_horizontal_tunnel, apply_room_to_map, apply_vertical_tunnel, generate_rooms, Room,
};
use super::{Layer, TileType};

use rand::Rng;
pub struct RoomLayer {
    pub layer: Layer<TileType>,
    pub rooms: Vec<Room>,
}

impl RoomLayer {
    pub fn new(width: usize, height: usize, room_amount: usize) -> RoomLayer {
        let mut layer = Layer::new(TileType::Wall, width, height);
        let rooms = generate_rooms(width, height, room_amount);

        let mut rng = rand::thread_rng();

        let mut prev_x: i32 = 0;
        let mut prev_y: i32 = 0;

        for room in rooms.iter() {
            apply_room_to_map(&mut layer, &room);

            let (new_x, new_y) = room.center();
            if rng.gen_range(1..=2) == 1 {
                apply_horizontal_tunnel(&mut layer, prev_x, new_x, prev_y);
                apply_vertical_tunnel(&mut layer, prev_y, new_y, new_x);
            } else {
                apply_vertical_tunnel(&mut layer, prev_y, new_y, prev_x);
                apply_horizontal_tunnel(&mut layer, prev_x, new_x, new_y);
            }
            (prev_x, prev_y) = (new_x, new_y);
        }

        RoomLayer { layer, rooms }
    }
}
