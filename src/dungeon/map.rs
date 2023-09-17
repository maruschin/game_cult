mod enums;
mod layer;
mod matrix;
mod room;
mod room_layer;

pub use enums::TileType;
pub use layer::Layer;
pub use room_layer::RoomLayer;

pub const DUNGEON_ROW: usize = 20;
pub const DUNGEON_COLUMN: usize = 10;

pub struct Map {
    pub room_layer: RoomLayer,
}

impl Map {
    pub fn new() -> Self {
        const ROOM_AMOUNT: usize = 6;
        const ROOM_LAYER_SCALE: f32 = 4.;
        let room_layer = RoomLayer::new(DUNGEON_ROW, DUNGEON_COLUMN, ROOM_LAYER_SCALE, ROOM_AMOUNT);
        Map { room_layer }
    }
}
