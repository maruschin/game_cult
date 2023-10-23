#[derive(PartialEq, Copy, Clone)]

pub enum WallType {
    Left,
    Right,
    Top,
    Bottom,
    LeftRight,
    TopBottom,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Empthy,
    Wall(WallType),
    Floor,
    Path,
}
