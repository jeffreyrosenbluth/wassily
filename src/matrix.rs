use num_traits::{Float, One, AsPrimitive, Zero};
/// A zero indexed row-major matrix.
/// Allows acces to elements of matrix A by A[i][j]
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    pub data: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn new<U: AsPrimitive<usize>>(rows: U, cols: U, data: Vec<T>) -> Self {
        Self { rows: rows.as_(), cols: cols.as_(), data }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    fn get_index(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    pub fn generate<F, U: AsPrimitive<usize>>(rows: U, cols: U, generator: F) -> Self
    where
        F: Fn(usize, usize) -> T,
    {
        let mut data: Vec<T> = vec![];
        for r in 0..rows.as_() {
            for c in 0..cols.as_() {
                data.push(generator(r, c))
            }
        }
        Matrix { rows: rows.as_(), cols: cols.as_(), data }
    }

    pub fn get_ref(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.rows && col < self.cols {
            Some(&self.data[self.get_index(row, col)])
        } else {
            None
        }
    }

    pub fn put(&mut self, row: usize, col: usize, item: T) -> bool {
        if row >= self.rows || col >= self.cols {
            false
        } else {
            let idx = self.get_index(row, col);
            self.data[idx] = item;
            true
        }
    }
}

impl<T> Matrix<T>
where
    T: Clone + Copy,
{
    pub fn fill(rows: usize, cols: usize, datum: T) -> Self {
        let data = vec![datum; rows * cols];
        Self { rows, cols, data }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<T> {
        if row < self.rows && col < self.cols {
            let idx = self.get_index(row, col);
            Some(self.data[idx])
        } else {
            None
        }
    }
}

impl<T> Matrix<T>
where
    T: Zero + Clone,
{
    pub fn zeros<U: AsPrimitive<usize>>(rows: U, cols: U) -> Self {
        let data = vec![T::zero(); rows.as_() * cols.as_()];
        Self { rows: rows.as_(), cols: cols.as_(), data }
    }
}

impl<T> Matrix<T>
where
    T: One + Clone,
{
    pub fn ones<U: AsPrimitive<usize>>(rows: U, cols: U) -> Self {
        let data = vec![T::one(); rows.as_() * cols.as_()];
        Self { rows: rows.as_(), cols: cols.as_(), data }
    }
}

impl<T> Matrix<T>
where
    T: Float,
{
    pub fn convolve(&self, kernel: &Matrix<T>) -> Matrix<T> {
        let mut m: Matrix<T> = Matrix {
            rows: self.rows,
            cols: self.cols,
            data: self.data.clone(),
        };
        let k = kernel.rows / 2;
        for i in k..self.rows - k {
            for j in k..self.cols - k {
                let mut acc = T::zero();
                for r in 0..kernel.rows {
                    for c in 0..kernel.cols {
                        acc = acc + self[i - k + r][j - k + c] * kernel[r][c];
                    }
                }
                m[i][j] = acc;
            }
        }
        m
    }
}

impl<T> Index<usize> for Matrix<T> {
    type Output = [T];
    fn index(&self, index: usize) -> &Self::Output {
        let start = index * self.cols;
        &self.data[start..start + self.cols]
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let start = index * self.cols;
        &mut self.data[start..start + self.cols]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gen_test() {
        let m = Matrix::generate(2, 3, |i, j| (i, j));
        assert_eq!(m.data, vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2)]);
    }

    #[test]
    fn get_test() {
        let m = Matrix::generate(2, 3, |i, j| (i, j));
        assert_eq!(m.get(1, 1), Some((1, 1)));
        assert_eq!(m.get(2, 1), None);
        assert_eq!(m.get(0, 3), None);
    }

    #[test]
    fn get_ref_test() {
        let m = Matrix::generate(2, 3, |i, j| (i, j));
        assert_eq!(m.get_ref(1, 1), Some(&(1, 1)));
        assert_eq!(m.get_ref(2, 1), None);
        assert_eq!(m.get_ref(0, 3), None);
    }

    #[test]
    fn put_test() {
        let mut m = Matrix::generate(2, 3, |i, j| (i, j));
        assert_eq!(m.put(1, 1, (5, 5)), true);
        assert_eq!(m.get(1, 1), Some((5, 5)));
    }

    #[test]
    fn fill_test() {
        let m = Matrix::fill(1, 2, true);
        assert_eq!(m.data, vec![true, true]);
    }

    #[test]
    fn index_test() {
        let m = Matrix::generate(2, 3, |i, j| (i, j));
        assert_eq!(m[1][1], (1, 1));
    }

    #[test]
    fn indexmut_test() {
        let mut m = Matrix::generate(2, 3, |i, j| (i, j));
        m[1][1] = (5, 5);
        assert_eq!(m[1][1], (5, 5));
    }

    #[test]
    fn convolve_test() {
        let m = Matrix::<f32>::ones(5, 5);
        let k = Matrix::<f32>::ones(3, 3);
        let c = m.convolve(&k);
        assert_eq!(c[0][0], 1.0);
        assert_eq!(c[1][1], 9.0);
    }
}
