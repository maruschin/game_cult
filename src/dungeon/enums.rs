#[derive(Clone, Copy, PartialEq)]
pub enum WallType {
    Left,
    Right,
    Top,
    Bottom,
    LeftRight,
    TopBottom,
}

#[derive(Clone, Copy, PartialEq)]
pub enum DoorType {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Clone, Copy, PartialEq)]
pub enum CornerType {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Empthy,
    Wall(WallType),
    Corner(CornerType),
    Floor,
    Path,
    Door(DoorType),
}
