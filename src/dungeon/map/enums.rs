#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Empthy,
    WallLeft,
    WallRight,
    WallLeftRight,
    WallTop,
    WallBottom,
    WallTopBottom,
    Floor,
    Path,
}
