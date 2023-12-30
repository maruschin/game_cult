use crate::dungeon::enums::{CornerType, DoorType, FloorType, TileType, WallType};

use super::base::Layer;
use super::room::RoomLayer;

pub struct WallLayer<const ROW: usize, const COLUMN: usize> {
    pub layer: Layer<TileType, ROW, COLUMN>,
}

impl<const ROW: usize, const COLUMN: usize> WallLayer<ROW, COLUMN> {
    pub fn new(scale: f32, wall_layer: RoomLayer<ROW, COLUMN>) -> WallLayer<ROW, COLUMN> {
        let mut layer = Layer::new(TileType::Empthy, scale);

        for (i, j, el) in wall_layer.layer.windows_2x2() {
            match el {
                | [[FloorType::Empthy, FloorType::Empthy], [FloorType::Empthy, FloorType::Room]] => {
                    layer[(i + 1, j + 1)] =
                        TileType::Wall(WallType::InternalCorner(CornerType::BottomLeft));
                }
                | [[FloorType::Empthy, FloorType::Empthy], [FloorType::Room, FloorType::Empthy]] => {
                    layer[(i + 1, j)] =
                        TileType::Wall(WallType::InternalCorner(CornerType::BottomRight));
                }
                | [[FloorType::Empthy, FloorType::Room], [FloorType::Empthy, FloorType::Empthy]] => {
                    layer[(i, j + 1)] =
                        TileType::Wall(WallType::InternalCorner(CornerType::TopLeft));
                }
                | [[FloorType::Room, FloorType::Empthy], [FloorType::Empthy, FloorType::Empthy]] => {
                    layer[(i, j)] = TileType::Wall(WallType::InternalCorner(CornerType::TopRight));
                }
                | _ => {}
            }
        }

        for (i, j, el) in wall_layer.layer.windows_1x3() {
            match el {
                | [[FloorType::Empthy, FloorType::Room, FloorType::Room]] => {
                    if layer[(i, j + 1)] == TileType::Empthy {
                        layer[(i, j + 1)] = TileType::Wall(WallType::Left)
                    }
                }
                | [[FloorType::Room, FloorType::Room, FloorType::Empthy]] => {
                    if layer[(i, j + 1)] == TileType::Empthy {
                        layer[(i, j + 1)] = TileType::Wall(WallType::Right)
                    }
                }
                | _ => {}
            }
        }

        for (i, j, el) in wall_layer.layer.windows_3x1() {
            match el {
                | [[FloorType::Empthy], [FloorType::Room], [FloorType::Room]] => {
                    if layer[(i + 1, j)] == TileType::Empthy {
                        layer[(i + 1, j)] = TileType::Wall(WallType::Bottom)
                    }
                }
                | [[FloorType::Room], [FloorType::Room], [FloorType::Empthy]] => {
                    if layer[(i + 1, j)] == TileType::Empthy {
                        layer[(i + 1, j)] = TileType::Wall(WallType::Top)
                    }
                }
                | _ => {}
            }
        }

        for (i, j, el) in wall_layer.layer.clone().windows_2x1() {
            match el {
                | [[FloorType::Room], [FloorType::Path]] => {
                    layer[(i, j)] = TileType::Door(DoorType::Right);
                }
                | [[FloorType::Path], [FloorType::Room]] => {
                    layer[(i + 1, j)] = TileType::Door(DoorType::Left);
                }
                | [[FloorType::Empthy], [_]] => {
                    //layer[(i, j)] = TileType::Wall(WallType::Bottom)
                }
                | [[_], [FloorType::Empthy]] => {
                    //layer[(i + 1, j)] = TileType::Wall(WallType::Top)
                }
                | _ => {}
            }
        }

        for (i, j, el) in wall_layer.layer.clone().windows_1x2() {
            match el {
                | [[FloorType::Room, FloorType::Path]] => {
                    layer[(i, j)] = TileType::Door(DoorType::Bottom)
                }
                | [[FloorType::Path, FloorType::Room]] => {
                    layer[(i, j + 1)] = TileType::Door(DoorType::Top)
                }
                | [[FloorType::Room | FloorType::Path, FloorType::Empthy]] => {
                    //layer[(i, j)] = TileType::Wall(WallType::Right)
                }
                | [[FloorType::Empthy, FloorType::Room | FloorType::Path]] => {
                    //layer[(i, j)] = TileType::Wall(WallType::Left)
                }
                | _ => {}
            }
        }

        WallLayer { layer }
    }
}
