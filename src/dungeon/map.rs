mod layer;
mod room_layer;
mod wall_layer;

pub use super::{TileType, WallType};
pub use room_layer::RoomLayer;
pub use wall_layer::WallLayer;

pub struct Map<const COLUMN: usize, const ROW: usize> {
    pub room_layer: RoomLayer<COLUMN, ROW>,
    pub wall_layer: WallLayer<COLUMN, ROW>,
}

impl<const COLUMN: usize, const ROW: usize> Map<COLUMN, ROW> {
    pub fn new() -> Self {
        const ROOM_AMOUNT: usize = 6;
        const ROOM_LAYER_SCALE: f32 = 4.;
        let room_layer = RoomLayer::new(ROOM_LAYER_SCALE, ROOM_AMOUNT);
        let wall_layer = WallLayer::new(ROOM_LAYER_SCALE, room_layer.clone());
        Map {
            room_layer,
            wall_layer,
        }
    }
}
