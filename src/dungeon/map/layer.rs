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
    width: usize,
    length: usize,
    scale: f32,
}

impl<T> Layer<T>
where
    T: Copy,
{
    pub fn new(default: T, width: usize, length: usize, scale: f32) -> Layer<T> {
        let tiles = Matrix::from_vec(width, length, vec![default; width * length]);
        let coordinates = Matrix::from_fn(width, length, |i, j| {
            ((i as f32) * scale, (j as f32) * scale)
        });
        Layer {
            tiles,
            coordinates,
            width,
            length,
            scale,
        }
    }

    pub fn set(&mut self, width_coord: usize, length_coord: usize, value: T) {
        self.tiles[(width_coord, length_coord)] = value;
    }

    pub fn get(&mut self, width_coord: usize, length_coord: usize) -> T {
        self.tiles[(width_coord, length_coord)]
    }

    pub fn iter(&self) -> Zip<Iter<'_, (f32, f32)>, Iter<'_, T>> {
        zip(self.coordinates.iter(), self.tiles.iter())
    }
}
