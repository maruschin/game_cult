// Copyright 2016 generic-matrix-rs Developers
// https://github.com/gifnksm/generic-matrix-rs
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Manipulations and data types that represent 2d matrix.

use std::mem::swap;
use std::ops::{Index, IndexMut};
use std::slice::Iter;

/// 2D matrix.
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct Matrix<T> {
    pub row: usize,
    pub column: usize,
    pub data: Vec<T>,
}

#[allow(dead_code)]
impl<T> Matrix<T> {
    /// Creates a new `Matrix`.
    pub fn from_fn<F>(row: usize, column: usize, f: F) -> Matrix<T>
    where
        F: Fn(usize, usize) -> T,
    {
        Matrix {
            row,
            column,
            data: (0..row * column)
                .map(|i| f(i / column, i % column))
                .collect(),
        }
    }

    /// Creates a new `Matrix` from vector.
    pub fn from_vec(row: usize, column: usize, data: Vec<T>) -> Matrix<T> {
        assert_eq!(row * column, data.len());
        Matrix { row, column, data }
    }

    /// Returns the matrix's row and column.
    pub fn size(&self) -> (usize, usize) {
        (self.row, self.column)
    }
    /// Returns the matrix's row.
    pub fn row(&self) -> usize {
        self.row
    }
    /// Returns the matrix's column.
    pub fn column(&self) -> usize {
        self.column
    }

    /// Returns iterator over all matrix data elements.
    pub fn iter(&self) -> Iter<'_, T> {
        self.data.iter()
    }

    /// Transposes the matrix in-place.
    pub fn trans_in_place(&mut self) {
        if self.row == self.column {
            // easy case of square matrix
            for i in 0..self.row {
                for j in 0..i {
                    self.data.swap(i * self.column + j, j * self.column + i);
                }
            }
        } else {
            // easy case of either dimension being zero or one
            swap(&mut self.row, &mut self.column);
            if self.row > 1 && self.column > 1 {
                // hard case of non-square matrix with both dimensions at least two
                let mut skip_bitmap = vec![0u32; (self.row * self.column + 31) / 32];

                for i in 0..self.row {
                    for j in 0..self.column {
                        // within this block is where bugs are most likely to be
                        let original_this = i * self.column + j;
                        let mut this = original_this;
                        let mut other = j * self.row + i;
                        // make sure each rotation is performed exactly once
                        while original_this < other
                            && skip_bitmap[this / 32] & (1u32 << (this % 32)) == 0
                        {
                            self.data.swap(this, other);
                            skip_bitmap[this / 32] |= 1u32 << (this % 32);
                            this = other;
                            other = (this % self.column) * self.row + (this / self.column);
                        }
                    }
                }
            }
        }
    }
}

#[allow(dead_code)]
impl<'a, T: Clone> Matrix<T> {
    /// Returns transpose of the matrix.
    pub fn trans(&self) -> Matrix<T> {
        Matrix::from_fn(self.column(), self.row(), |i, j| self[(j, i)].clone())
    }

    /// Returns an iterator as a matrix with the specified number of rows and columns.
    pub fn windows(&self, row: usize, column: usize) -> MatrixWindows<T> {
        MatrixWindows {
            matrix: self,
            i: 0,
            j: 0,
            window_row: row,
            window_column: column,
        }
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, (i, j): (usize, usize)) -> &T {
        assert!(i < self.row && j < self.column);
        &self.data[i * self.column + j]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut T {
        assert!(i < self.row && j < self.column);
        &mut self.data[i * self.column + j]
    }
}

pub struct MatrixWindows<'a, T>
where
    T: 'a,
{
    matrix: &'a Matrix<T>,
    i: usize,
    j: usize,
    window_row: usize,
    window_column: usize,
}

impl<'a, T> Iterator for MatrixWindows<'a, T>
where
    T: Clone,
{
    type Item = Matrix<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.i <= self.matrix.row() - self.window_row
            && self.j <= self.matrix.column() - self.window_column
        {
            Some(Matrix::from_fn(
                self.window_row,
                self.window_column,
                |i, j| &self.matrix[(i + self.i, j + self.j)],
            ))
        } else {
            None
        };
        if self.j < self.matrix.column() - self.window_column {
            self.j += 1;
        } else {
            self.j = 0;
            self.i += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix;

    #[test]
    fn from_vec() {
        let mat = Matrix::from_vec(2, 3, vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2)]);
        for i in 0..mat.row() {
            for j in 0..mat.column() {
                assert_eq!((i, j), mat[(i, j)]);
            }
        }
    }

    #[test]
    fn index() {
        let mat = Matrix::from_fn(3, 5, |i, j| (i, j));
        for i in 0..mat.row() {
            for j in 0..mat.column() {
                assert_eq!((i, j), mat[(i, j)]);
            }
        }
    }

    #[test]
    fn trans() {
        let mut square = Matrix::from_vec(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(
            square.trans(),
            Matrix::from_vec(3, 3, vec![1, 4, 7, 2, 5, 8, 3, 6, 9])
        );
        square.trans_in_place();
        assert_eq!(
            square,
            Matrix::from_vec(3, 3, vec![1, 4, 7, 2, 5, 8, 3, 6, 9])
        );

        let mut vector = Matrix::from_vec(3, 1, vec![1, 2, 3]);
        assert_eq!(vector.trans(), Matrix::from_vec(1, 3, vec![1, 2, 3]));
        vector.trans_in_place();
        assert_eq!(vector, Matrix::from_vec(1, 3, vec![1, 2, 3]));

        let mut rect_2_3 = Matrix::from_vec(3, 2, vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(
            rect_2_3.trans(),
            Matrix::from_vec(2, 3, vec![1, 3, 5, 2, 4, 6])
        );
        rect_2_3.trans_in_place();
        assert_eq!(rect_2_3, Matrix::from_vec(2, 3, vec![1, 3, 5, 2, 4, 6]));

        let mut rect_5_2 = Matrix::from_vec(2, 5, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(
            rect_5_2.trans(),
            Matrix::from_vec(5, 2, vec![1, 6, 2, 7, 3, 8, 4, 9, 5, 10])
        );
        rect_5_2.trans_in_place();
        assert_eq!(
            rect_5_2,
            Matrix::from_vec(5, 2, vec![1, 6, 2, 7, 3, 8, 4, 9, 5, 10])
        );

        let mut rect_5_3 = Matrix::from_vec(
            3,
            5,
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        );
        assert_eq!(
            rect_5_3.trans(),
            Matrix::from_vec(
                5,
                3,
                vec![1, 6, 11, 2, 7, 12, 3, 8, 13, 4, 9, 14, 5, 10, 15]
            )
        );
        rect_5_3.trans_in_place();
        assert_eq!(
            rect_5_3,
            Matrix::from_vec(
                5,
                3,
                vec![1, 6, 11, 2, 7, 12, 3, 8, 13, 4, 9, 14, 5, 10, 15]
            )
        );
    }
    #[test]
    fn windows2x2() {
        let square = Matrix::from_vec(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let mut square_windows = square.windows(2, 2);

        assert_eq!(
            square_windows.next(),
            Some(Matrix::from_vec(2, 2, vec![&1, &2, &4, &5]))
        );
        assert_eq!(
            square_windows.next(),
            Some(Matrix::from_vec(2, 2, vec![&2, &3, &5, &6]))
        );
        assert_eq!(
            square_windows.next(),
            Some(Matrix::from_vec(2, 2, vec![&4, &5, &7, &8]))
        );
        assert_eq!(
            square_windows.next(),
            Some(Matrix::from_vec(2, 2, vec![&5, &6, &8, &9]))
        );
        assert_eq!(square_windows.next(), None);
    }
    #[test]
    fn windows1x2() {
        let square = Matrix::from_vec(2, 2, vec![1, 2, 3, 4]);
        let mut square_windows = square.windows(1, 2);

        assert_eq!(
            square_windows.next(),
            Some(Matrix::from_vec(1, 2, vec![&1, &2]))
        );
        assert_eq!(
            square_windows.next(),
            Some(Matrix::from_vec(1, 2, vec![&3, &4]))
        );
        assert_eq!(square_windows.next(), None);
    }
    #[test]
    fn windows2x1() {
        let square = Matrix::from_vec(2, 2, vec![1, 2, 3, 4]);
        let mut square_windows = square.windows(2, 1);

        assert_eq!(
            square_windows.next(),
            Some(Matrix::from_vec(2, 1, vec![&1, &3]))
        );
        assert_eq!(
            square_windows.next(),
            Some(Matrix::from_vec(2, 1, vec![&2, &4]))
        );
        assert_eq!(square_windows.next(), None);
    }
}
