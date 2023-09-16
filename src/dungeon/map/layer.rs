use core::slice::Iter;

// Структура в виде вектора копируемых объектов создана для реализации логического слоя карты.
// Какие объекты там расположены, видима ли карта для игрока, исследована ли она им и т.д.
#[derive(Debug)]
pub struct Layer<T: Copy> {
    map: Vec<T>,
    width: usize,
    length: usize,
}

impl<T> Layer<T>
where
    T: Copy,
{
    pub fn new(default: T, width: usize, length: usize) -> Layer<T> {
        Layer {
            map: vec![default; width * length],
            width,
            length,
        }
    }

    fn wide_and_length_coord_to_idx(&self, width_coord: usize, length_coord: usize) -> usize {
        (width_coord * self.length) + length_coord
    }

    pub fn idx_to_width_and_length_coord(&self, idx: usize) -> (i32, i32) {
        (
            (idx as i32) / (self.length as i32),
            (idx as i32) % (self.length as i32),
        )
    }

    pub fn set(&mut self, width_coord: usize, length_coord: usize, value: T) {
        let idx = self.wide_and_length_coord_to_idx(width_coord, length_coord);
        self.map[idx] = value;
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.map.iter()
    }
}
