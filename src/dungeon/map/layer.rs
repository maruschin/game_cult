use core::slice::Iter;

use super::matrix::Matrix;
use std::iter::{zip, Zip};

// Структура в виде вектора копируемых объектов создана для реализации логического слоя карты.
// Какие объекты там расположены, видима ли карта для игрока, исследована ли она им и т.д.
#[allow(dead_code)]
#[derive(Debug)]
pub struct Layer<T: Copy> {
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

    pub fn iter(&self) -> Zip<Iter<'_, (f32, f32)>, Iter<'_, T>> {
        zip(self.coordinates.iter(), self.tiles.iter())
    }
}
