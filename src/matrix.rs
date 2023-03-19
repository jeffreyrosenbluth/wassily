/// A zero indexed row-major matrix.
/// Allows acces to elements of matrix A by A[i][j]
///
use num_traits::{zero, AsPrimitive, Float, One, Zero};
use std::ops::{Add, Div, Index, IndexMut, Mul};

#[derive(Debug)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    pub data: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn new<U: AsPrimitive<usize>>(rows: U, cols: U, data: Vec<T>) -> Self {
        assert_eq!(rows.as_() * cols.as_(), data.len());
        Self {
            rows: rows.as_(),
            cols: cols.as_(),
            data,
        }
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
        Matrix {
            rows: rows.as_(),
            cols: cols.as_(),
            data,
        }
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

    pub fn valid<U: Into<usize>>(&self, row: U, col: U) -> bool {
        row.into() < self.rows() && col.into() < self.cols()
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

    /// Insert a column at position n.
    pub fn insert_col(&self, n: usize, column: Vec<T>) -> Self {
        assert_eq!(column.len(), self.rows());
        Matrix::generate(self.rows(), self.cols() + 1, |r, c| {
            if c < n {
                self[r][c]
            } else if c == n {
                column[r]
            } else {
                self[r][c - 1]
            }
        })
    }

    /// Insert a row at position n.
    pub fn insert_row(&self, n: usize, row: Vec<T>) -> Self {
        assert_eq!(row.len(), self.cols());
        Matrix::generate(self.rows() + 1, self.cols(), |r, c| {
            if r < n {
                self[r][c]
            } else if r == n {
                row[c]
            } else {
                self[r - 1][c]
            }
        })
    }

    pub fn transpose(&self) -> Self {
        Matrix::generate(self.cols(), self.rows(), |r, c| self[c][r])
    }
}

impl<T> Matrix<T>
where
    T: Zero + Clone,
{
    pub fn zeros<U: AsPrimitive<usize>>(rows: U, cols: U) -> Self {
        let data = vec![T::zero(); rows.as_() * cols.as_()];
        Self {
            rows: rows.as_(),
            cols: cols.as_(),
            data,
        }
    }
}

impl<T> Matrix<T>
where
    T: One + Clone,
{
    pub fn ones<U: AsPrimitive<usize>>(rows: U, cols: U) -> Self {
        let data = vec![T::one(); rows.as_() * cols.as_()];
        Self {
            rows: rows.as_(),
            cols: cols.as_(),
            data,
        }
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

impl<T> Mul<T> for &Matrix<T>
where
    T: Mul<Output = T> + Zero + Copy,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut m: Matrix<T> = Matrix::fill(self.rows(), self.cols(), zero());
        for r in 0..self.rows() {
            for c in 0..self.cols() {
                m[r][c] = self[r][c] * rhs;
            }
        }
        m
    }
}

impl<T> Div<T> for &Matrix<T>
where
    T: Div<Output = T> + Zero + Copy,
{
    type Output = Matrix<T>;

    fn div(self, rhs: T) -> Self::Output {
        let mut m: Matrix<T> = Matrix::fill(self.rows(), self.cols(), zero());
        for r in 0..self.rows() {
            for c in 0..self.cols() {
                m[r][c] = self[r][c] / rhs;
            }
        }
        m
    }
}

impl<T> Mul<&Vec<T>> for &Matrix<T>
where
    T: Add<Output = T> + Mul<Output = T> + Zero + Copy,
{
    type Output = Vec<T>;

    fn mul(self, rhs: &Vec<T>) -> Self::Output {
        assert_eq!(self.cols(), rhs.len());
        let mut v: Vec<T> = vec![];
        for r in 0..self.rows() {
            v.push(
                self[r]
                    .iter()
                    .zip(rhs)
                    .fold(zero(), |accum: T, item| accum + *item.0 * *item.1),
            );
        }
        v
    }
}

impl<T> Mul<&Matrix<T>> for &Matrix<T>
where
    T: Add<Output = T> + Mul<Output = T> + Zero + Copy,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: &Matrix<T>) -> Self::Output {
        assert_eq!(self.cols(), rhs.rows());
        let mut m: Matrix<T> = Matrix::fill(self.rows(), rhs.cols(), zero());
        for r in 0..self.rows() {
            for c in 0..rhs.cols() {
                let mut a = zero();
                for i in 0..self.cols() {
                    a = a + self[r][i] * rhs[i][c];
                }
                m[r][c] = a;
            }
        }
        m
    }
}

impl<T> PartialEq for Matrix<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.rows == other.rows && self.cols == other.cols && self.data == other.data
    }
}

impl<T> Eq for Matrix<T> where T: Eq {}

#[cfg(test)]
mod tests {
    use std::vec;

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

    #[test]
    fn mul_test() {
        let m = Matrix::new(2, 2, vec![1, 2, 3, 4]);
        assert_eq!(&m * &vec![5, 10], vec![25, 55]);
    }

    #[test]
    fn mul_mat_test() {
        let m1 = Matrix::new(3, 2, vec![1, 2, 3, 4, 5, 6]);
        let m2 = Matrix::new(2, 2, vec![5, 10, 50, 100]);
        assert_eq!((&m1 * &m2).data, vec![105, 210, 215, 430, 325, 650,]);
    }

    #[test]
    fn mul_scalar_test() {
        let m = Matrix::new(2, 2, vec![1, 2, 3, 4]);
        assert_eq!((&m * 2).data, vec![2, 4, 6, 8]);
    }

    #[test]
    fn insert_col_test() {
        let m = Matrix::new(2, 2, vec![1, 2, 3, 4]);
        let m1 = m.insert_col(1, vec![5, 5]);
        assert_eq!(m1.data, vec![1, 5, 2, 3, 5, 4]);
    }

    #[test]
    fn insert_row_test() {
        let m = Matrix::new(2, 2, vec![1, 2, 3, 4]);
        let m1 = m.insert_row(1, vec![5, 5]);
        assert_eq!(m1.data, vec![1, 2, 5, 5, 3, 4]);
    }

    #[test]
    fn transpose_test() {
        let m = Matrix::new(2, 2, vec![1, 2, 3, 4]);
        let m1 = m.transpose();
        assert_eq!(m1.data, vec![1, 3, 2, 4]);
    }
}
