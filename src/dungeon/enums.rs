#[derive(Clone, Copy, PartialEq)]
pub enum CornerType {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Clone, Copy, PartialEq)]
pub enum WallType {
    Left,
    Right,
    Top,
    Bottom,
    InternalCorner(CornerType),
    ExternalCorner(CornerType),
}

#[derive(Clone, Copy, PartialEq)]
pub enum DoorType {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Empthy,
    Wall(WallType),
    Floor,
    Path,
    Door(DoorType),
}
