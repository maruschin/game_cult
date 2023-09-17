mod enums;
mod layer;
mod matrix;
mod room;
mod room_layer;

pub use enums::TileType;
pub use layer::Layer;
pub use room_layer::RoomLayer;

pub const DUNGEON_WIDTH: usize = 20;
pub const DUNGEON_LENGTH: usize = 10;

pub struct Map {
    pub room_layer: RoomLayer,
}

impl Map {
    pub fn new() -> Self {
        const MAX_ROOMS: i32 = 6;
        let room_layer = RoomLayer::new(
            DUNGEON_WIDTH as usize,
            DUNGEON_LENGTH as usize,
            MAX_ROOMS as usize,
        );
        Map { room_layer }
    }
}
