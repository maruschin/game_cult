mod layer;
mod room_layer;

pub use super::{CornerType, TileType, WallType};
pub use layer::Layer;
pub use room_layer::RoomLayer;

pub struct Map<const COLUMN: usize, const ROW: usize> {
    pub room_layer: RoomLayer<COLUMN, ROW>,
}

impl<const COLUMN: usize, const ROW: usize> Map<COLUMN, ROW> {
    pub fn new() -> Self {
        const ROOM_AMOUNT: usize = 6;
        const ROOM_LAYER_SCALE: f32 = 4.;
        let room_layer = RoomLayer::new(ROOM_LAYER_SCALE, ROOM_AMOUNT);
        Map { room_layer }
    }
}
