use std::ops::Add;

use crate::vector::{VectorItem, Vector};

struct Matrix<T: VectorItem, const R:usize, const C:usize> {
    items: [Vector<T, R>; C]
}

impl<T: VectorItem, const R:usize, const C:usize> {
    pub fn new() -> Matrix<T, R, C> {
        Matrix { items: [Vector::new::<T, R>(); C] }
    }
    pub fn get(self, r_i:usize) -> Vector<T, R> {
        self.items[r_i]
    }
}

impl<T: VectorItem, const R:usize, const C:usize> Add for Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;
    fn add(self, rhs: Self) -> Self::Output {
        let mut new = Matrix::new();
        for i in 0..C {
            new[i] = self.get(i) + rhs.get(i);
        }
        new
    }
}
