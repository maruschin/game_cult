use core::slice::Iter;

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

    fn idx(&self, x: usize, y: usize) -> usize {
        (x * self.height) + y
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        let idx = self.idx(x, y);
        self.map[idx] = value;
    }

    pub fn get(&self, x: usize, y: usize) -> T {
        self.map[self.idx(x, y)]
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.map.iter()
    }
}
