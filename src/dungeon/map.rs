mod enums;
mod layer;
mod room;

pub use enums::TileType;
pub use layer::Layer;
pub use room::{apply_horizontal_tunnel, apply_room_to_map, apply_vertical_tunnel, Room};

use rand::Rng;

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
