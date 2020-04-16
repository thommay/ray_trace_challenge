use anyhow::*;
use num::Float;
use std::fmt::Debug;
use std::ops::{AddAssign, Mul, Neg, Sub};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd)]
pub struct Matrix<T>
where
    T: Mul<Output = T>
        + Sub<Output = T>
        + Neg<Output = T>
        + Float
        + AddAssign
        + Copy
        + Clone
        + Default
        + Debug,
{
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> Matrix<T>
where
    T: Mul<Output = T>
        + Sub<Output = T>
        + Neg<Output = T>
        + Float
        + AddAssign
        + Copy
        + Clone
        + Default
        + Debug,
{
    pub fn new(rows: usize, cols: usize) -> Self {
        Self::from_iter(rows, cols, (0..).map(|_| T::default()))
    }

    pub fn from_iter(rows: usize, cols: usize, data: impl IntoIterator<Item = T>) -> Matrix<T> {
        Matrix {
            rows,
            cols,
            data: {
                let data: Vec<_> = data.into_iter().take(rows * cols).collect();
                assert_eq!(data.len(), rows * cols);
                data
            },
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if (col < self.cols) && (row < self.rows) {
            Some(&self.data[col + row * self.cols])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if (col < self.cols) && (row < self.rows) {
            Some(&mut self.data[col + row * self.cols])
        } else {
            None
        }
    }
    pub fn set(&mut self, row: usize, col: usize, item: T) -> bool {
        if let Some(p) = self.get_mut(row, col) {
            *p = item;
            return true;
        } else {
            return false;
        }
    }

    pub fn get_row(&self, row: usize) -> Option<impl Iterator<Item = &T>> {
        if row < self.rows {
            Some((0..self.cols).map(move |col| self.get(row, col).unwrap()))
        } else {
            None
        }
    }

    pub fn get_col(&self, col: usize) -> Option<impl Iterator<Item = &T>> {
        if col < self.cols {
            Some((0..self.rows).map(move |row| self.get(row, col).unwrap()))
        } else {
            None
        }
    }

    fn transpose(&self) -> Matrix<T> {
        let out = Self {
            rows: self.rows,
            cols: self.cols,
            data: {
                let mut data = Vec::with_capacity(self.rows * self.cols);
                for row in 0..self.rows {
                    for val in self.get_col(row).unwrap().cloned() {
                        data.push(val);
                    }
                }
                data
            },
        };
        out
    }

    fn determinant(&self) -> T {
        if self.rows == 2 && self.cols == 2 {
            return self.data[0] * self.data[3] - self.data[1] * self.data[2];
        }
        let mut d: T = Default::default();
        for (i, item) in self.get_row(0).unwrap().enumerate() {
            let c = self.cofactor(0, i);
            d += c * *item;
        }
        d
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix<T> {
        Self {
            rows: self.rows - 1,
            cols: self.cols - 1,
            data: {
                let mut d = Vec::new();
                for r in 0..self.rows {
                    if r == row {
                        continue;
                    }
                    for c in 0..self.cols {
                        if c == col {
                            continue;
                        }
                        let n = self.data[c + r * self.cols].clone();
                        d.push(n);
                    }
                }
                d
            },
        }
    }

    fn minor(&self, row: usize, col: usize) -> T {
        let m = self.submatrix(row, col);
        m.determinant()
    }

    fn cofactor(&self, row: usize, col: usize) -> T {
        let point = row + col;
        let m = self.minor(row, col);
        if point % 2 == 0 {
            m
        } else {
            -m
        }
    }

    fn invertible(&self) -> bool {
        !self.determinant().is_zero()
    }

    fn inverse(&self) -> Result<Matrix<T>> {
        if !self.invertible() {
            return Err(anyhow!("matrix isn't invertible"));
        }
        let mut s = Self::new(self.rows, self.cols);
        let det = self.determinant();
        for row in 0..self.rows {
            for col in 0..self.cols {
                let c = self.cofactor(row, col);
                // using row for the column and vice versa does the transpose
                s.set(col, row, c / det.clone());
            }
        }
        Ok(s)
    }

    fn round(&self, factor: T) -> Matrix<T> {
        Self {
            rows: self.rows,
            cols: self.cols,
            data: {
                self.data
                    .iter()
                    .map(move |&x| (x * factor).round() / factor)
                    .collect()
            },
        }
    }
}

impl<T> Mul<Matrix<T>> for Matrix<T>
where
    T: Mul<Output = T>
        + Sub<Output = T>
        + Neg<Output = T>
        + Float
        + AddAssign
        + Copy
        + Clone
        + Default
        + Debug,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        assert!(self.cols == rhs.rows);
        Matrix {
            rows: self.rows,
            cols: rhs.cols,
            data: {
                let mut data = Vec::with_capacity(self.rows * rhs.cols);
                for row in 0..self.rows {
                    for col in 0..rhs.cols {
                        let row = self.get_row(row).unwrap();
                        let col = rhs.get_col(col).unwrap();
                        let mut iter = row.zip(col);
                        let (a, b) = iter.next().unwrap();
                        let mut acc = *a * *b;
                        for (a, b) in iter {
                            acc += *a * *b;
                        }
                        data.push(acc);
                    }
                }
                data
            },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::matrix::Matrix;

    fn identity() -> Matrix<f32> {
        let mut identity = Matrix::new(4, 4);
        identity.set(0, 0, 1.0);
        identity.set(1, 1, 1.0);
        identity.set(2, 2, 1.0);
        identity.set(3, 3, 1.0);
        identity
    }

    #[test]
    fn test_identity() {
        let i = identity();
        let data = vec![
            0.0, 1.0, 2.0, 4.0, 1.0, 2.0, 4.0, 8.0, 2.0, 4.0, 8.0, 16.0, 4.0, 8.0, 16.0, 32.0,
        ];
        let d = Matrix::from_iter(4, 4, data);
        assert_eq!(d.clone() * i, d);
    }

    #[test]
    fn test_multiply() {
        let first = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ];

        let second = vec![
            -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0,
        ];

        let d = Matrix::from_iter(4, 4, first);
        let i = Matrix::from_iter(4, 4, second);

        let res = vec![
            20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0, 26.0,
            46.0, 42.0,
        ];
        let r = Matrix::from_iter(4, 4, res);
        assert_eq!(d * i, r);
    }

    #[test]
    fn test_transpose() {
        let i = vec![
            0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0,
        ];
        let i = Matrix::from_iter(4, 4, i);
        let r = vec![
            0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0,
        ];
        let r = Matrix::from_iter(4, 4, r);
        assert_eq!(i.transpose(), r);
    }

    #[test]
    fn test_identity_transpose() {
        assert_eq!(identity().transpose(), identity());
    }

    #[test]
    fn test_two_two_determinant() {
        let i = Matrix::from_iter(2, 2, vec![1.0, 5.0, -3.0, 2.0]);
        assert_eq!(i.determinant(), 17.0);
    }

    #[test]
    fn test_three_three_determinant() {
        let i = Matrix::from_iter(3, 3, vec![1.0, 2.0, 6.0, -5.0, 8.0, -4.0, 2.0, 6.0, 4.0]);
        assert_eq!(i.cofactor(0, 0), 56.0);
        assert_eq!(i.cofactor(0, 1), 12.0);
        assert_eq!(i.cofactor(0, 2), -46.0);
        assert_eq!(i.determinant(), -196.0);
    }

    #[test]
    fn test_four_four_determinant() {
        let i = Matrix::from_iter(
            4,
            4,
            vec![
                -2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0,
                -9.0,
            ],
        );
        assert_eq!(i.cofactor(0, 0), 690.0);
        assert_eq!(i.cofactor(0, 1), 447.0);
        assert_eq!(i.cofactor(0, 2), 210.0);
        assert_eq!(i.cofactor(0, 3), 51.0);
        assert_eq!(i.determinant(), -4071.0);
    }

    #[test]
    fn test_three_submatrix() {
        let i = Matrix::from_iter(3, 3, vec![1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0]);
        let p = Matrix::from_iter(2, 2, vec![-3.0, 2.0, 0.0, 6.0]);
        assert_eq!(i.submatrix(0, 2), p);
    }
    #[test]
    fn test_four_submatrix() {
        let i = Matrix::from_iter(
            4,
            4,
            vec![
                -6.0, 1.0, 1.0, 6.0, -8.0, 5.0, 8.0, 6.0, -1.0, 0.0, 8.0, 2.0, -7.0, 1.0, -1.0, 1.0,
            ],
        );
        let p = Matrix::from_iter(3, 3, vec![-6.0, 1.0, 6.0, -8.0, 8.0, 6.0, -7.0, -1.0, 1.0]);
        assert_eq!(i.submatrix(2, 1), p);
    }

    #[test]
    fn test_three_minor() {
        let i = Matrix::from_iter(3, 3, vec![3.0, 5.0, 0.0, 2.0, 1.0, -7.0, 6.0, -1.0, 5.0]);
        assert_eq!(i.minor(1, 0), 25.0);
    }

    #[test]
    fn test_three_cofactor() {
        let i = Matrix::from_iter(3, 3, vec![3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);
        assert_eq!(i.cofactor(0, 0), -12.0);
        assert_eq!(i.cofactor(1, 0), -25.0);
    }

    #[test]
    fn test_four_invertible() {
        let good = Matrix::from_iter(
            4,
            4,
            vec![
                6.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 6.0, 4.0, -9.0, 3.0, -7.0, 9.0, 1.0, 7.0, -6.0,
            ],
        );
        let bad = Matrix::from_iter(
            4,
            4,
            vec![
                -4.0, 2.0, -2.0, -3.0, 9.0, 6.0, 2.0, 6.0, 0.0, -5.0, 1.0, -5.0, 0.0, 0.0, 0.0, 0.0,
            ],
        );
        assert!(good.invertible());
        assert!(!bad.invertible());
    }

    #[test]
    fn test_inverse_first() {
        let a = Matrix::from_iter(
            4,
            4,
            vec![
                -5.0, 2.0, 6.0, -8.0, 1.0, -5.0, 1.0, 8.0, 7.0, 7.0, -6.0, -7.0, 1.0, -3.0, 7.0,
                4.0,
            ],
        );
        let b = Matrix::from_iter(
            4,
            4,
            vec![
                0.21805, 0.45113, 0.24060, -0.04511, -0.80827, -1.45677, -0.44361, 0.52068,
                -0.07895, -0.22368, -0.05263, 0.19737, -0.52256, -0.81391, -0.30075, 0.30639,
            ],
        );
        assert_eq!(a.determinant(), 532.0);
        assert_eq!(a.cofactor(2, 3), -160.0);
        // assert_eq!(b.get(3, 2).unwrap(), &(-160.0 / 532.0));
        assert_eq!(a.cofactor(3, 2), 105.0);
        // assert_eq!(b.get(2, 3).unwrap(), &(105.0 / 532.0));
        assert_eq!(a.inverse().unwrap().round(100000.0), b);
    }

    #[test]
    fn test_inverse_second() {
        let a = Matrix::from_iter(
            4,
            4,
            vec![
                8.0, -5.0, 9.0, 2.0, 7.0, 5.0, 6.0, 1.0, -6.0, 0.0, 9.0, 6.0, -3.0, 0.0, -9.0, -4.0,
            ],
        );
        let b = Matrix::from_iter(
            4,
            4,
            vec![
                -0.15385, -0.15385, -0.28205, -0.53846, -0.07692, 0.12308, 0.02564, 0.03077,
                0.35897, 0.35897, 0.43590, 0.92308, -0.69231, -0.69231, -0.76923, -1.92308,
            ],
        );
        assert_eq!(a.inverse().unwrap().round(100000.0), b);
    }

    #[test]
    fn test_inverse_third() {
        let a = Matrix::from_iter(
            4,
            4,
            vec![
                9.0, 3.0, 0.0, 9.0, -5.0, -2.0, -6.0, -3.0, -4.0, 9.0, 6.0, 4.0, -7.0, 6.0, 6.0,
                2.0,
            ],
        );
        let b = Matrix::from_iter(
            4,
            4,
            vec![
                -0.04074, -0.07778, 0.14444, -0.22222, -0.07778, 0.03333, 0.36667, -0.33333,
                -0.02901, -0.14630, -0.10926, 0.12963, 0.17778, 0.06667, -0.26667, 0.33333,
            ],
        );
        assert_eq!(a.inverse().unwrap().round(100000.0), b);
    }

    #[test]
    fn test_inverse_roundtrip() {
        let a = Matrix::from_iter(
            4,
            4,
            vec![
                3.0, -9.0, 7.0, 3.0, 3.0, -8.0, 2.0, -9.0, -4.0, 4.0, 4.0, 1.0, -6.0, 5.0, -1.0,
                1.0,
            ],
        );
        let b = Matrix::from_iter(
            4,
            4,
            vec![
                8.0, 2.0, 2.0, 2.0, 3.0, -1.0, 7.0, 0.0, 7.0, 0.0, 5.0, 4.0, 6.0, -2.0, 0.0, 5.0,
            ],
        );
        let c = a.clone() * b.clone();
        assert_eq!((c * b.inverse().unwrap()).round(100000f32), a)
    }
}
