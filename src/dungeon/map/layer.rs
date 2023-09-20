use std::ops::{Index, IndexMut};

// Структура в виде вектора копируемых объектов создана для реализации логического слоя карты.
// Какие объекты там расположены, видима ли карта для игрока, исследована ли она им и т.д.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Layer<T> {
    data: Vec<T>,
    pub row: usize,
    pub column: usize,
    pub scale: f32,
}

impl<T> Layer<T>
where
    T: Copy,
{
    pub fn new(default: T, row: usize, column: usize, scale: f32) -> Layer<T> {
        let data = vec![default; row * column];
        Layer {
            data,
            row,
            column,
            scale,
        }
    }
}

impl<T> Layer<T> {
    pub fn get_coordiante_by_index(&self, index: usize) -> (f32, f32) {
        let (i, j) = (index / self.column, index % self.column);
        ((i as f32) * self.scale, (j as f32) * self.scale)
    }

    pub fn len(&self) -> usize {
        return self.data.len();
    }

    pub fn iter(&self) -> LayerIterator<'_, T> {
        LayerIterator::new(self)
    }
}

impl<T> Index<(usize, usize)> for Layer<T> {
    type Output = T;

    fn index(&self, (i, j): (usize, usize)) -> &T {
        assert!(i < self.row && j < self.column);
        &self.data[i * self.column + j]
    }
}

impl<T> IndexMut<(usize, usize)> for Layer<T> {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut T {
        assert!(i < self.row && j < self.column);
        &mut self.data[i * self.column + j]
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
    type Item = (f32, f32, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.layer.len() {
            let (x, y) = self.layer.get_coordiante_by_index(self.index);
            let tile = &self.layer.data[self.index];
            self.index += 1;
            Some((x, y, tile))
        } else {
            None
        }
    }
}
