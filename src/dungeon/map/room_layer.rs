use super::room::{
    apply_length_tunnel, apply_room_to_map, apply_width_tunnel, generate_rooms, Room,
};
use super::{Layer, TileType};

use rand::Rng;
pub struct RoomLayer {
    pub layer: Layer<TileType>,
    pub rooms: Vec<Room>,
}

impl RoomLayer {
    pub fn new(width: usize, length: usize, room_amount: usize) -> RoomLayer {
        let mut layer = Layer::new(TileType::Wall, width, length, 4.);
        let rooms = generate_rooms(width, length, room_amount);

        let mut rng = rand::thread_rng();

        let mut prev_width_coord: i32 = 0;
        let mut prev_length_coord: i32 = 0;

        for room in rooms.iter() {
            apply_room_to_map(&mut layer, &room);

            let (new_width_coord, new_length_coord) = room.center();
            if rng.gen_range(1..=2) == 1 {
                apply_width_tunnel(
                    &mut layer,
                    prev_width_coord,
                    new_width_coord,
                    prev_length_coord,
                );
                apply_length_tunnel(
                    &mut layer,
                    new_width_coord,
                    prev_length_coord,
                    new_length_coord,
                );
            } else {
                apply_length_tunnel(
                    &mut layer,
                    prev_width_coord,
                    prev_length_coord,
                    new_length_coord,
                );
                apply_width_tunnel(
                    &mut layer,
                    prev_width_coord,
                    new_width_coord,
                    new_length_coord,
                );
            }
            (prev_width_coord, prev_length_coord) = (new_width_coord, new_length_coord);
        }

        RoomLayer { layer, rooms }
    }
}
