//! Manipulations and data types that represent 2d matrix.
//! Looked at https://github.com/gifnksm/generic-matrix-rs

use std::mem::swap;
use std::ops::{Index, IndexMut};
use std::slice::Iter;

/// 2D matrix.
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct Matrix<T> {
    row: usize,
    column: usize,
    data: Vec<T>,
}

#[allow(dead_code)]
impl<T> Matrix<T> {
    /// Creates a new `Matrix`.
    #[inline]
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
    #[inline]
    pub fn from_vec(row: usize, column: usize, data: Vec<T>) -> Matrix<T> {
        assert_eq!(row * column, data.len());
        Matrix { row, column, data }
    }

    /// Returns the matrix's row and column.
    #[inline]
    pub fn size(&self) -> (usize, usize) {
        (self.row(), self.column())
    }
    /// Returns the matrix's row.
    #[inline]
    pub fn row(&self) -> usize {
        self.row
    }
    /// Returns the matrix's column.
    #[inline]
    pub fn column(&self) -> usize {
        self.column
    }

    /// Returns iterator over all matrix elements.
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
impl<T: Clone> Matrix<T> {
    #[inline]
    /// Returns transpose of the matrix.
    pub fn trans(&self) -> Matrix<T> {
        Matrix::from_fn(self.column(), self.row(), |i, j| self[(j, i)].clone())
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    #[inline]
    fn index(&self, (i, j): (usize, usize)) -> &T {
        assert!(i < self.row() && j < self.column());
        &self.data[i * self.column() + j]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    #[inline]
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut T {
        assert!(i < self.row && j < self.column);
        &mut self.data[i * self.column + j]
    }
}
