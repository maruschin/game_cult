use std::ops::{Index, IndexMut};

// Структура в виде вектора копируемых объектов создана для реализации логического слоя карты.
// Какие объекты там расположены, видима ли карта для игрока, исследована ли она им и т.д.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Layer<T, const ROW: usize, const COLUMN: usize> {
    data: [[T; COLUMN]; ROW],
    pub scale: f32,
}

impl<T, const ROW: usize, const COLUMN: usize> Layer<T, ROW, COLUMN>
where
    T: Copy,
{
    pub fn new(default: T, scale: f32) -> Layer<T, ROW, COLUMN> {
        let data = [[default; COLUMN]; ROW];
        Layer { data, scale }
    }
}

impl<T, const ROW: usize, const COLUMN: usize> Layer<T, ROW, COLUMN> {
    pub fn get_coordiante(&self, i: usize, j: usize) -> (f32, f32) {
        ((i as f32) * self.scale, (j as f32) * self.scale)
    }

    pub fn iter(&self) -> LayerIterator<'_, T, ROW, COLUMN> {
        LayerIterator::new(self)
    }
}

impl<T, const ROW: usize, const COLUMN: usize> Index<(usize, usize)> for Layer<T, ROW, COLUMN> {
    type Output = T;

    fn index(&self, (i, j): (usize, usize)) -> &T {
        assert!(i < ROW && j < COLUMN);
        &self.data[i][j]
    }
}

impl<T, const ROW: usize, const COLUMN: usize> IndexMut<(usize, usize)> for Layer<T, ROW, COLUMN> {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut T {
        assert!(i < ROW && j < COLUMN);
        &mut self.data[i][j]
    }
}

pub struct LayerIterator<'a, T, const ROW: usize, const COLUMN: usize>
where
    T: 'a,
{
    layer: &'a Layer<T, ROW, COLUMN>,
    i: usize,
    j: usize,
}

impl<T, const ROW: usize, const COLUMN: usize> LayerIterator<'_, T, ROW, COLUMN> {
    fn new(layer: &Layer<T, ROW, COLUMN>) -> LayerIterator<'_, T, ROW, COLUMN> {
        LayerIterator { layer, i: 0, j: 0 }
    }
}

impl<'a, T, const ROW: usize, const COLUMN: usize> Iterator for LayerIterator<'a, T, ROW, COLUMN> {
    type Item = (f32, f32, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < ROW && self.j < COLUMN {
            let (x, y) = self.layer.get_coordiante(self.i, self.j);
            let tile = &self.layer.data[self.i][self.j];
            self.j += 1;
            if self.j == COLUMN && self.i < ROW {
                self.j = 0;
                self.i += 1;
            }
            Some((x, y, tile))
        } else {
            None
        }
    }
}
