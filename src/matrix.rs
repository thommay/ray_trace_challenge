use crate::vec3::TypedVec;
use anyhow::*;
use num::Float;
use std::fmt::{Debug, Display};
use std::ops::{AddAssign, Mul, Neg, Sub};

pub enum Axis {
    X,
    Y,
    Z,
}

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

    pub fn identity(size: usize) -> Matrix<T> {
        Self {
            rows: size,
            cols: size,
            data: {
                let mut d: Vec<T> = (0..size * size).map(|_| T::default()).collect();
                for r in 0..size {
                    for c in 0..size {
                        if r == c {
                            d[c + r * size] = T::one();
                        }
                    }
                }
                d
            },
        }
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

    pub fn translation(x: T, y: T, z: T) -> Matrix<T> {
        let mut i = Self::identity(4);
        i.set(0, 3, x);
        i.set(1, 3, y);
        i.set(2, 3, z);
        i
    }

    pub fn scaling(x: T, y: T, z: T) -> Matrix<T> {
        let mut i = Self::identity(4);
        i.set(0, 0, x);
        i.set(1, 1, y);
        i.set(2, 2, z);
        i
    }

    pub fn rotation(axis: Axis, distance: T) -> Matrix<T> {
        match axis {
            Axis::X => Self::rotate_x(distance),
            Axis::Y => Self::rotate_y(distance),
            Axis::Z => Self::rotate_z(distance),
        }
    }

    pub fn shearing(xy: T, xz: T, yx: T, yz: T, zx: T, zy: T) -> Matrix<T> {
        let mut m = Matrix::new(4, 4);
        m.set(0, 0, T::one());
        m.set(0, 1, xy);
        m.set(0, 2, xz);
        m.set(1, 0, yx);
        m.set(1, 1, T::one());
        m.set(1, 2, yz);
        m.set(2, 0, zx);
        m.set(2, 1, zy);
        m.set(2, 2, T::one());
        m.set(3, 3, T::one());
        m
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
            true
        } else {
            false
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

    pub(crate) fn transpose(&self) -> Matrix<T> {
        Self {
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
        }
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
                        let n = self.data[c + r * self.cols];
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

    pub(crate) fn inverse(&self) -> Result<Matrix<T>> {
        if !self.invertible() {
            return Err(anyhow!("matrix isn't invertible"));
        }
        let mut s = Self::new(self.rows, self.cols);
        let det = self.determinant();
        for row in 0..self.rows {
            for col in 0..self.cols {
                let c = self.cofactor(row, col);
                // using row for the column and vice versa does the transpose
                s.set(col, row, c / det);
            }
        }
        Ok(s)
    }

    fn rotate_x(distance: T) -> Matrix<T> {
        let mut m = Matrix::new(4, 4);
        m.set(0, 0, T::one());
        m.set(1, 1, distance.cos());
        m.set(1, 2, -distance.sin());
        m.set(2, 1, distance.sin());
        m.set(2, 2, distance.cos());
        m.set(3, 3, T::one());
        m
    }

    fn rotate_y(distance: T) -> Matrix<T> {
        let mut m = Matrix::new(4, 4);
        m.set(0, 0, distance.cos());
        m.set(0, 2, distance.sin());
        m.set(1, 1, T::one());
        m.set(2, 0, -distance.sin());
        m.set(2, 2, distance.cos());
        m.set(3, 3, T::one());
        m
    }

    fn rotate_z(distance: T) -> Matrix<T> {
        let mut m = Matrix::new(4, 4);
        m.set(0, 0, distance.cos());
        m.set(0, 1, -distance.sin());
        m.set(1, 0, distance.sin());
        m.set(1, 1, distance.cos());
        m.set(2, 2, T::one());
        m.set(3, 3, T::one());
        m
    }

    #[cfg(test)]
    pub(crate) fn round(&self, factor: T) -> Matrix<T> {
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
        + Debug
        + Display,
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

impl<T> Mul<TypedVec> for Matrix<T>
where
    T: Mul<Output = T>
        + Sub<Output = T>
        + Neg<Output = T>
        + Float
        + AddAssign
        + Copy
        + Clone
        + Default
        + Debug
        + Display
        + Into<f64>,
{
    type Output = TypedVec;

    fn mul(self, rhs: TypedVec) -> Self::Output {
        assert!(self.cols == 4);
        let vec = vec![rhs.x, rhs.y, rhs.z, rhs.w];
        TypedVec {
            x: { mul_int(&self, &vec, 0) },
            y: { mul_int(&self, &vec, 1) },
            z: { mul_int(&self, &vec, 2) },
            w: rhs.w,
            is: rhs.is,
        }
    }
}

fn mul_int<T>(m: &Matrix<T>, vec: &[f64], row: usize) -> f64
where
    T: Mul<Output = T>
        + Sub<Output = T>
        + Neg<Output = T>
        + Float
        + AddAssign
        + Copy
        + Clone
        + Default
        + Debug
        + Display
        + Into<f64>,
{
    let row = m.get_row(row).unwrap();
    let mut iter = row.zip(vec);
    let (a, b) = iter.next().unwrap();
    let mut acc: f64 = Into::<f64>::into(*a) * *b;
    for (a, b) in iter {
        acc += Into::<f64>::into(*a) * *b;
    }
    acc
}

impl<T> Mul<TypedVec> for &'_ Matrix<T>
where
    T: Mul<Output = T>
        + Sub<Output = T>
        + Neg<Output = T>
        + Float
        + AddAssign
        + Copy
        + Clone
        + Default
        + Debug
        + Display
        + Into<f64>,
{
    type Output = TypedVec;

    fn mul(self, rhs: TypedVec) -> Self::Output {
        assert!(self.cols == 4);
        let vec = vec![rhs.x, rhs.y, rhs.z, rhs.w];
        TypedVec {
            x: { mul_int(&self, &vec, 0) },
            y: { mul_int(&self, &vec, 1) },
            z: { mul_int(&self, &vec, 2) },
            w: rhs.w,
            is: rhs.is,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::matrix::{Axis, Matrix};
    use crate::vec3::TypedVec;

    #[test]
    fn test_identity() {
        let i = Matrix::identity(4);
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
        let out: Matrix<f64> = Matrix::identity(4);
        let out = out.transpose();
        assert_eq!(out, Matrix::identity(4));
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
        assert_eq!((c * b.inverse().unwrap()).round(100000f64), a)
    }

    #[test]
    fn test_translate_no_vector() {
        let i = Matrix::translation(5.0, -3.0, 2.0);
        let p = TypedVec::vector(-3f64, 4f64, 5f64);
        assert_eq!(i * p, p)
    }

    #[test]
    fn test_translate_matrix() {
        let i = Matrix::translation(5.0, -3.0, 2.0);
        let p = TypedVec::point(-3f64, 4f64, 5f64);
        assert_eq!(i * p, TypedVec::point(2f64, 1f64, 7f64))
    }

    #[test]
    fn test_inverse_translate_matrix() {
        let i = Matrix::translation(5.0, -3.0, 2.0).inverse().unwrap();
        let p = TypedVec::point(-3f64, 4f64, 5f64);
        assert_eq!(i * p, TypedVec::point(-8f64, 7f64, 3f64))
    }

    #[test]
    fn test_scaling_matrix() {
        let i = Matrix::scaling(2.0, 3.0, 4.0);
        let p = TypedVec::point(-4f64, 6f64, 8f64);
        assert_eq!(i * p, TypedVec::point(-8f64, 18f64, 32f64))
    }

    #[test]
    fn test_scaling_vector() {
        let i = Matrix::scaling(2.0, 3.0, 4.0);
        let p = TypedVec::vector(-4f64, 6f64, 8f64);
        assert_eq!(i * p, TypedVec::vector(-8f64, 18f64, 32f64))
    }

    #[test]
    fn test_inverse_scaling_matrix() {
        let i = Matrix::scaling(2.0, 3.0, 4.0).inverse().unwrap();
        let p = TypedVec::vector(-4f64, 6f64, 8f64);
        assert_eq!(i * p, TypedVec::vector(-2f64, 2f64, 2f64))
    }

    #[test]
    fn test_reflection() {
        let i = Matrix::scaling(-1.0, 1.0, 1.0).inverse().unwrap();
        let p = TypedVec::point(2f64, 3f64, 4f64);
        assert_eq!(i * p, TypedVec::point(-2f64, 3f64, 4f64))
    }

    #[test]
    fn test_rotate_x() {
        let p = TypedVec::point(0f64, 1f64, 0f64);
        let q = Matrix::rotation(Axis::X, std::f64::consts::PI / 4f64);
        let h = Matrix::rotation(Axis::X, std::f64::consts::PI / 2f64);
        assert_eq!(
            (q.clone() * p).round(10000f64),
            TypedVec::point(0f64, 2f64.sqrt() / 2f64, 2f64.sqrt() / 2f64).round(10000f64)
        );
        assert_eq!(
            (h * p).round(10000f64),
            TypedVec::point(0f64, 0f64, 1f64).round(10000f64)
        );
        let inv = q.inverse().unwrap();
        assert_eq!(
            (inv * p).round(10000f64),
            TypedVec::point(0f64, 2f64.sqrt() / 2f64, -(2f64.sqrt() / 2f64)).round(10000f64)
        );
    }

    #[test]
    fn test_rotate_y() {
        let p = TypedVec::point(0f64, 0f64, 1f64);
        let q = Matrix::rotation(Axis::Y, std::f64::consts::PI / 4f64);
        let h = Matrix::rotation(Axis::Y, std::f64::consts::PI / 2f64);
        assert_eq!(
            (q.clone() * p).round(10000f64),
            TypedVec::point(2f64.sqrt() / 2f64, 0f64, 2f64.sqrt() / 2f64).round(10000f64)
        );
        assert_eq!(
            (h * p).round(10000f64),
            TypedVec::point(1f64, 0f64, 0f64).round(10000f64)
        );
    }

    #[test]
    fn test_rotate_z() {
        let p = TypedVec::point(0f64, 1f64, 0f64);
        let q = Matrix::rotation(Axis::Z, std::f64::consts::PI / 4f64);
        let h = Matrix::rotation(Axis::Z, std::f64::consts::PI / 2f64);
        assert_eq!(
            (q.clone() * p).round(10000f64),
            TypedVec::point(-(2f64.sqrt() / 2f64), 2f64.sqrt() / 2f64, 0f64).round(10000f64)
        );
        assert_eq!(
            (h * p).round(10000f64),
            TypedVec::point(-1f64, 0f64, 0f64).round(10000f64)
        );
    }

    #[test]
    fn test_shearing() {
        let sxy = Matrix::shearing(1f64, 0f64, 0f64, 0f64, 0f64, 0f64);
        let p = TypedVec::point(2f64, 3f64, 4f64);
        assert_eq!(
            (sxy * p).round(10000f64),
            TypedVec::point(5f64, 3f64, 4f64).round(10000f64)
        );

        let sxz = Matrix::shearing(0f64, 1f64, 0f64, 0f64, 0f64, 0f64);
        assert_eq!(
            (sxz * p).round(10000f64),
            TypedVec::point(6f64, 3f64, 4f64).round(10000f64)
        );

        let syx = Matrix::shearing(0f64, 0f64, 1f64, 0f64, 0f64, 0f64);
        assert_eq!(
            (syx * p).round(10000f64),
            TypedVec::point(2f64, 5f64, 4f64).round(10000f64)
        );

        let syz = Matrix::shearing(0f64, 0f64, 0f64, 1f64, 0f64, 0f64);
        assert_eq!(
            (syz * p).round(10000f64),
            TypedVec::point(2f64, 7f64, 4f64).round(10000f64)
        );

        let szx = Matrix::shearing(0f64, 0f64, 0f64, 0f64, 1f64, 0f64);
        assert_eq!(
            (szx * p).round(10000f64),
            TypedVec::point(2f64, 3f64, 6f64).round(10000f64)
        );

        let szy = Matrix::shearing(0f64, 0f64, 0f64, 0f64, 0f64, 1f64);
        assert_eq!(
            (szy * p).round(10000f64),
            TypedVec::point(2f64, 3f64, 7f64).round(10000f64)
        );
    }

    #[test]
    fn test_transforms_sequence() {
        let p = TypedVec::point(1f64, 0f64, 1f64);
        let a = Matrix::rotation(Axis::X, std::f64::consts::PI / 2f64);
        let b = Matrix::scaling(5f64, 5f64, 5f64);
        let c = Matrix::translation(10f64, 5f64, 7f64);

        let p2 = a.clone() * p;
        assert_eq!(
            p2.round(10000f64),
            TypedVec::point(1f64, -1f64, 0f64).round(10000f64)
        );
        let p3 = b.clone() * p2;
        assert_eq!(
            p3.round(10000f64),
            TypedVec::point(5f64, -5f64, 0f64).round(10000f64)
        );
        let p4 = c.clone() * p3;
        assert_eq!(
            p4.round(10000f64),
            TypedVec::point(15f64, 0f64, 7f64).round(10000f64)
        );

        let t = c * b * a;
        assert_eq!(
            (t * p).round(10000f64),
            TypedVec::point(15f64, 0f64, 7f64).round(10000f64)
        );
    }
}
