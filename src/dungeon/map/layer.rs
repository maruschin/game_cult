use std::ops::{Index, IndexMut};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
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

    pub fn row(&self) -> usize {
        ROW
    }

    pub fn column(&self) -> usize {
        COLUMN
    }

    pub fn iter(&self) -> LayerIterator<'_, T, ROW, COLUMN> {
        LayerIterator::new(self)
    }

    pub fn windows_2x1(&self) -> LayerWindows<'_, T, ROW, COLUMN, 2, 1> {
        LayerWindows::new(self)
    }

    pub fn windows_1x2(&self) -> LayerWindows<'_, T, ROW, COLUMN, 1, 2> {
        LayerWindows::new(self)
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

pub struct LayerWindows<
    'a,
    T,
    const ROW: usize,
    const COLUMN: usize,
    const WINDOW_ROW: usize,
    const WINDOW_COLUMN: usize,
> where
    T: 'a,
{
    layer: &'a Layer<T, ROW, COLUMN>,
    i: usize,
    j: usize,
}

impl<
        T,
        const ROW: usize,
        const COLUMN: usize,
        const WINDOW_ROW: usize,
        const WINDOW_COLUMN: usize,
    > LayerWindows<'_, T, ROW, COLUMN, WINDOW_ROW, WINDOW_COLUMN>
{
    fn new(
        layer: &Layer<T, ROW, COLUMN>,
    ) -> LayerWindows<'_, T, ROW, COLUMN, WINDOW_ROW, WINDOW_COLUMN> {
        LayerWindows { layer, i: 0, j: 0 }
    }
}

impl<
        'a,
        T: Copy,
        const ROW: usize,
        const COLUMN: usize,
        const WINDOW_ROW: usize,
        const WINDOW_COLUMN: usize,
    > Iterator for LayerWindows<'a, T, ROW, COLUMN, WINDOW_ROW, WINDOW_COLUMN>
{
    type Item = (usize, usize, [[&'a T; WINDOW_COLUMN]; WINDOW_ROW]);

    fn next(&mut self) -> Option<Self::Item> {
        let max_row = ROW + 1 - WINDOW_ROW;
        let max_column = COLUMN + 1 - WINDOW_COLUMN;
        if self.i < max_row && self.j < max_column {
            let mut window = [[&self.layer.data[0][0]; WINDOW_COLUMN]; WINDOW_ROW];
            for i_window in 0..WINDOW_ROW {
                for j_window in 0..WINDOW_COLUMN {
                    window[i_window][j_window] =
                        &self.layer.data[self.i + i_window][self.j + j_window];
                }
            }
            let result = Some((self.i, self.j, window));
            self.j += 1;
            if self.j == max_column && self.i < max_row {
                self.j = 0;
                self.i += 1;
            }
            result
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator() -> () {
        let mut layer = Layer::<f32, 3, 3>::new(0.0, 1.0);
        for i in 0..layer.row() {
            for j in 0..layer.column() {
                layer[(i, j)] = (i as f32) * (j as f32) * layer.scale * layer.scale;
            }
        }

        let mut new_layer = Layer::<f32, 3, 3>::new(0.0, 1.0);
        for (i, j, ..) in new_layer.clone().iter() {
            new_layer[(i as usize, j as usize)] = i * j
        }

        assert_eq!(layer, new_layer);
    }

    #[test]
    fn test_windows1x2() -> () {
        let mut layer = Layer::<f32, 3, 4>::new(1.0, 1.0);
        for i in 0..layer.row() {
            for j in 0..layer.column() {
                layer[(i, j)] = (i as f32) * (j as f32) * layer.scale * layer.scale;
            }
        }

        assert_eq!(
            layer.windows_1x2().collect::<Vec<_>>(),
            vec![
                (0, 0, [[&0.0, &0.0]]),
                (0, 1, [[&0.0, &0.0]]),
                (0, 2, [[&0.0, &0.0]]),
                (1, 0, [[&0.0, &1.0]]),
                (1, 1, [[&1.0, &2.0]]),
                (1, 2, [[&2.0, &3.0]]),
                (2, 0, [[&0.0, &2.0]]),
                (2, 1, [[&2.0, &4.0]]),
                (2, 2, [[&4.0, &6.0]]),
            ]
        )
    }
}
