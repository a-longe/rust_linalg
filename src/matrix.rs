use std::any;
use std::{fmt::Debug, iter::zip};
use std::{ops::Add, ops::Mul};

use crate::vector::{Vector, VectorItem};

#[derive(Copy, Clone, PartialEq)]
pub struct Matrix<T: VectorItem, const R: usize, const C: usize> {
    items: [Vector<T, R>; C],
}

// Macro for creating a matrix literal
// Usage: mat![1, 2; 3, 4] => Matrix::from([[1, 3], [2, 4]])
#[macro_export]
macro_rules! mat {
    // Match rows separated by semicolons, elements separated by commas
    ($($($val:expr),+);+) => {
        {
            // Collect all rows into a nested array
            let rows = [$( [$($val),+] ),+];

            // Transpose: rows become columns since Vector is a column
            Matrix::from_rows(rows)
        }
    };
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct AugmentedMatrix<T: VectorItem, const R: usize, const C1: usize, const C2: usize> {
    left: Matrix<T, R, C1>,
    right: Matrix<T, R, C2>,
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

impl<T: VectorItem, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn from_rows(rows: [[T; C]; R]) -> Self {
        let mut cols = [[T::default(); R]; C];

        for col in 0..C {
            for row in 0..R {
                cols[col][row] = rows[row][col];
            }
        }

        Matrix::from(cols)
    }
}

impl<T: VectorItem, const R: usize, const C: usize> Debug for Matrix<T, R, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Matrix<{},{},{}>", any::type_name::<T>(), R, C)?;
        for row in 0..R {
            for col in 0..C {
                write!(f, "{:?} ", self.get(row, col).unwrap_or(T::one()))?;
            }
            writeln!(f)?;
        }
        Ok(())
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
    i8, i16, i32, i64, i128, isize, f32, f64
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

// Row Operations
impl<T: VectorItem, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn row_swap(&mut self, row1_i: usize, row2_i: usize) {
        if row1_i >= R || row2_i >= R {
            panic!("Row index out of bounds");
        }
        for c_i in 0..C {
            let temp = self
                .get(row1_i, c_i)
                .expect("Matrix is larger than type suggests");
            self.set(
                row1_i,
                c_i,
                self.get(row2_i, c_i)
                    .expect("Matrix is larger than type suggests"),
            );
            self.set(row2_i, c_i, temp);
        }
    }
    pub fn row_mult(&mut self, row_i: usize, scalar: T) {
        for c_i in 0..C {
            let temp = self
                .get(row_i, c_i)
                .expect("Matrix is larger than type suggests");
            self.set(row_i, c_i, temp * scalar);
        }
    }
    pub fn row_add(&mut self, row1_i: usize, row2_i: usize, scalar: T) {
        for c_i in 0..C {
            let temp = self
                .get(row1_i, c_i)
                .expect("Matrix is larger than type suggests");
            self.set(
                row1_i,
                c_i,
                temp + self
                    .get(row2_i, c_i)
                    .expect("Matrix is larger than type suggests")
                    * scalar,
            );
        }
    }
}

// Augmented Matrices
impl<T: VectorItem, const R: usize, const C1: usize, const C2: usize>
    From<(Matrix<T, R, C1>, Matrix<T, R, C2>)> for AugmentedMatrix<T, R, C1, C2> {
    fn from((left, right): (Matrix<T, R, C1>, Matrix<T, R, C2>)) -> Self {
        Self { left, right }
    }
}

impl<T: VectorItem, const R: usize, const C1: usize, const C2: usize> AugmentedMatrix<T, R, C1, C2> {
    pub fn get_left(&self) -> &Matrix<T, R, C1> {
        &self.left
    }
    pub fn get_right(&self) -> &Matrix<T, R, C2> {
        &self.right
    }
    pub fn get_mut_left(&mut self) -> &mut Matrix<T, R, C1> {
        &mut self.left
    }
    pub fn get_mut_right(&mut self) -> &mut Matrix<T, R, C2> {
        &mut self.right
    }
    pub fn row_swap(&mut self, row1_i: usize, row2_i: usize) {
        self.left.row_swap(row1_i, row2_i);
        self.right.row_swap(row1_i, row2_i);
    }
    pub fn row_add(&mut self, row1_i: usize, row2_i: usize, scalar: T) {
        self.left.row_add(row1_i, row2_i, scalar);
        self.right.row_add(row1_i, row2_i, scalar);
    }
    pub fn row_mult(&mut self, row_i: usize, scalar: T) {
        self.left.row_mult(row_i, scalar);
        self.right.row_mult(row_i, scalar);
    }
    pub fn reduce_left(&mut self) {
        let mut current_row = 0;

        for col in 0..C1 {
            if current_row >= R {
                break;
            }

            // Find a non-zero pivot in this column starting from current_row
            let mut pivot_row_opt = None;
            for row in current_row..R {
                let val = self.left.get(row, col).unwrap();
                if val != T::zero() {
                    pivot_row_opt = Some(row);
                    break;
                }
            }

            // If no pivot found in this column, skip to next column
            let pivot_row = match pivot_row_opt {
                Some(r) => r,
                None => continue,
            };

            // Swap the pivot row to current_row if needed
            if pivot_row != current_row {
                self.row_swap(pivot_row, current_row);
            }

            // Scale the pivot row so the pivot element becomes 1
            let pivot_val = self.left.get(current_row, col).unwrap();
            let scale_factor = T::one() / pivot_val;
            self.row_mult(current_row, scale_factor);

            // Eliminate all other elements in this column
            for row in 0..R {
                if row == current_row {
                    continue;
                }
                let element = self.left.get(row, col).unwrap();
                if element != T::zero() {
                    // row_add subtracts scalar * current_row from row
                    // We want: row -= element * current_row
                    self.row_add(row, current_row, -element);
                }
            }

            current_row += 1;
        }
    }
}

// Inverse
impl<T: VectorItem, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn inverse(&self) -> Option<Matrix<T, R, C>> {
        let mut aug = AugmentedMatrix::from((*self, Matrix::identity()));
        aug.reduce_left();
        if aug.left == Matrix::identity() {
            Some(aug.right)
        } else {
            None
        }
    }
}
