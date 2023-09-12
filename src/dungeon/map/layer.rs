use core::iter::Enumerate;
use core::slice::Iter;

// Структура в виде вектора копируемых объектов создана для реализации логического слоя карты.
// Какие объекты там расположены, видима ли карта для игрока, исследована ли она им и т.д.
#[derive(Debug)]
pub struct Layer<T: Copy> {
    map: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Layer<T>
where
    T: Copy,
{
    pub fn new(default: T, width: usize, height: usize) -> Layer<T> {
        Layer {
            map: vec![default; width * height],
            width,
            height,
        }
    }

    fn xy_to_idx(&self, x: usize, y: usize) -> usize {
        (x * self.height) + y
    }

    pub fn idx_to_xy(&self, idx: usize) -> (i32, i32) {
        (
            (idx as i32) / (self.width as i32),
            (idx as i32) % (self.width as i32),
        )
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        let idx = self.xy_to_idx(x, y);
        self.map[idx] = value;
    }

    pub fn get(&self, x: usize, y: usize) -> T {
        self.map[self.xy_to_idx(x, y)]
    }

    pub fn iter(&self) -> Enumerate<Iter<'_, T>> {
        self.map.iter().enumerate()
    }
}
