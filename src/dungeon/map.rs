mod enums;
mod layer;
mod room;
mod room_layer;

pub use enums::TileType;
pub use layer::Layer;
pub use room::{
    apply_horizontal_tunnel, apply_room_to_map, apply_vertical_tunnel, generate_rooms, Room,
};
pub use room_layer::RoomLayer;

pub const DUNGEON_WIDE: usize = 20;
pub const DUNGEON_HEIGHT: usize = 10;

pub struct Map {
    pub room_layer: RoomLayer,
}

impl Map {
    pub fn new() -> Self {
        const MAX_ROOMS: i32 = 4;
        let room_layer = RoomLayer::new(
            DUNGEON_WIDE as usize,
            DUNGEON_HEIGHT as usize,
            MAX_ROOMS as usize,
        );
        Map { room_layer }
    }
}
