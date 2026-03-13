use std::{fmt::Debug, iter::zip, slice::Iter};
use std::{ops::Add, ops::Sub, ops::Mul};
use std::any;

use crate::vector::{Vector, VectorItem};

#[derive(Copy, Clone, PartialEq)]
pub struct Matrix<T:VectorItem, const R:usize, const C:usize> {
    items: [Vector<T, R>; C]
}

impl<T: VectorItem, const R:usize, const C:usize> Matrix<T, R, C> {
    pub fn new() -> Matrix<T, R, C> {
        Matrix { items: [Vector::new(); C] }
    }
    pub fn get_cols(&self) -> Iter<'_, Vector<T, R>> {
        self.items.iter()
    }
    pub fn set_col(&mut self, c_i: usize, new_val: Vector<T, R>) -> bool {
        if c_i >= C { return false; }
        self.items[c_i] = new_val;
        true
    }
}

impl<T: VectorItem, const R:usize, const C:usize> From<[[T; R]; C]> for Matrix<T, R, C> {
    fn from(value: [[T; R]; C]) -> Self {
        Matrix { items: value.map(|vec| Vector::from(vec))}
    }
}

impl<T: VectorItem, const R:usize, const C:usize> Debug for Matrix<T, R, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("Matrix<{},{},{}>", any::type_name::<T>(), R, C))
            .field("items", &self.items)
            .finish()
    }
}

impl<T: VectorItem, const R:usize, const C:usize> Add for Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;
    fn add(self, rhs: Self) -> Self::Output {
        let mut new = Matrix::new();
        for (i, (c1, c2)) in zip(self.get_cols(), rhs.get_cols()).enumerate() {
            new.set_col(i, *c1 + *c2);
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
impl_mat_scalar_mul!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);
