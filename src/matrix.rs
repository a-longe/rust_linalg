use std::any;
use std::{fmt::Debug, iter::zip};
use std::{ops::Add, ops::Mul};

use crate::vector::{Vector, VectorItem};

#[derive(Copy, Clone, PartialEq)]
pub struct Matrix<T: VectorItem, const R: usize, const C: usize> {
    items: [Vector<T, R>; C],
}

impl<T: VectorItem, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn new() -> Matrix<T, R, C> {
        Matrix {
            items: [Vector::new(); C],
        }
    }
    pub fn get(&self, r_i: usize, c_i: usize) -> Option<T> {
        if c_i >= C {
            return None;
        }
        self.items[c_i].get(r_i)
    }
    pub fn get_col(&self, c_i: usize) -> Option<Vector<T, R>> {
        if c_i >= C {
            return None;
        }
        Some(self.items[c_i])
    }
    pub fn get_row(&self, r_i: usize) -> Option<Vector<T, C>> {
        if r_i >= R {
            return None;
        }
        self.get_rows().nth(r_i)
    }
    pub fn set(&mut self, r_i: usize, c_i: usize, new_val: T) -> bool {
        if c_i >= C {
            return false;
        }
        self.items[c_i].set(r_i, new_val)
    }
    pub fn get_cols(&self) -> impl Iterator<Item = Vector<T, R>> + '_ {
        self.items.iter().copied()
    }
    pub fn get_rows(&self) -> impl Iterator<Item = Vector<T, C>> + '_ {
        (0..R).map(|r_i| {
            let mut row_vec: Vector<T, C> = Vector::new();
            for c_i in 0..C {
                row_vec.set(
                    c_i,
                    self.get(r_i, c_i)
                        .expect("Matrix is larger than type suggests"),
                );
            }
            row_vec
        })
    }
    pub fn set_col(&mut self, c_i: usize, new_val: Vector<T, R>) -> bool {
        if c_i >= C {
            return false;
        }
        self.items[c_i] = new_val;
        true
    }
    pub fn get_transpose(&self) -> Matrix<T, C, R> {
        let mut transpose: Matrix<T, C, R> = Matrix::new();
        for r_i in 0..R {
            for c_i in 0..C {
                transpose.set(
                    c_i,
                    r_i,
                    self.get(r_i, c_i)
                        .expect("Matrix is larger than type suggests"),
                );
            }
        }
        transpose
    }
    pub fn identity() -> Matrix<T, R, C> {
        let mut mat = Matrix::new();
        for i in 0..R {
            mat.set(i, i, <T as VectorItem>::one());
        }
        mat
    }
}

impl<T: VectorItem, const R: usize, const C: usize> From<[[T; R]; C]> for Matrix<T, R, C> {
    fn from(value: [[T; R]; C]) -> Self {
        Matrix {
            items: value.map(|vec| Vector::from(vec)),
        }
    }
}

impl<T: VectorItem, const R: usize, const C: usize> Debug for Matrix<T, R, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("Matrix<{},{},{}>", any::type_name::<T>(), R, C))
            .field("items", &self.items)
            .finish()
    }
}

impl<T: VectorItem, const R: usize, const C: usize> Add for Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;
    fn add(self, rhs: Self) -> Self::Output {
        let mut new = Matrix::new();
        for (i, (c1, c2)) in zip(self.get_cols(), rhs.get_cols()).enumerate() {
            new.set_col(i, c1 + c2);
        }
        new
    }
}

macro_rules! impl_mat_scalar_mul {
    ($($t:ty),*) => {
        $(
            // right hand scalar mult
            impl<const R: usize, const C:usize> Mul<Matrix<$t, R, C>> for $t {
                type Output = Matrix<$t, R, C>;

                fn mul(self, rhs: Matrix<$t, R, C>) -> Self::Output {
                    let mut new: Matrix<$t, R, C> = Matrix::new();
                    for i in 0..C {
                        new.set_col(i, self * rhs.items[i]);
                    }
                    new
                }
            }
            // left hand scalar mult
            impl<const R: usize, const C:usize> Mul<$t> for Matrix<$t, R, C> {
                type Output = Matrix<$t, R, C>;

                fn mul(self, rhs: $t) -> Self::Output {
                    rhs * self
                }
            }
        )
*};}
impl_mat_scalar_mul!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
);

impl<T: VectorItem, const R: usize, const S: usize, const C: usize> Mul<Matrix<T, S, C>>
    for Matrix<T, R, S>
{
    type Output = Matrix<T, R, C>;
    fn mul(self, rhs: Matrix<T, S, C>) -> Self::Output {
        let mut new = Matrix::new();
        for (r_i, row) in self.get_rows().enumerate() {
            for (c_i, col) in rhs.get_cols().enumerate() {
                new.set(r_i, c_i, row.dot(col));
            }
        }
        new
    }
}
