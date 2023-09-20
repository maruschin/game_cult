use super::matrix::Matrix;

// Структура в виде вектора копируемых объектов создана для реализации логического слоя карты.
// Какие объекты там расположены, видима ли карта для игрока, исследована ли она им и т.д.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Layer<T> {
    tiles: Matrix<T>,
    coordinates: Matrix<(f32, f32)>,
    pub row: usize,
    pub column: usize,
    pub scale: f32,
}

impl<T> Layer<T>
where
    T: Copy,
{
    pub fn new(default: T, row: usize, column: usize, scale: f32) -> Layer<T> {
        let tiles = Matrix::from_vec(row, column, vec![default; row * column]);
        let coordinates =
            Matrix::from_fn(row, column, |i, j| ((i as f32) * scale, (j as f32) * scale));
        Layer {
            tiles,
            coordinates,
            row,
            column,
            scale,
        }
    }

    pub fn set(&mut self, i: usize, j: usize, value: T) {
        self.tiles[(i, j)] = value;
    }

    pub fn get(&mut self, i: usize, j: usize) -> T {
        self.tiles[(i, j)]
    }

    pub fn iter(&self) -> LayerIterator<'_, T> {
        LayerIterator::new(self)
    }
}

pub struct LayerIterator<'a, T>
where
    T: 'a,
{
    layer: &'a Layer<T>,
    index: usize,
}

impl<T> LayerIterator<'_, T> {
    fn new(layer: &Layer<T>) -> LayerIterator<'_, T> {
        LayerIterator { layer, index: 0 }
    }
}

impl<'a, T> Iterator for LayerIterator<'a, T> {
    type Item = (&'a f32, &'a f32, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.layer.coordinates.data.len() {
            let (x, y) = &self.layer.coordinates.data[self.index];
            let tile = &self.layer.tiles.data[self.index];
            self.index += 1;
            Some((x, y, tile))
        } else {
            None
        }
    }
}
