pub mod matrix;
pub mod vector;

#[cfg(test)]
mod vector_tests {
    use crate::vector::*;

    #[test]
    fn create_vector_new() {
        let _: Vector<i32, 5> = Vector::new();
    }

    #[test]
    fn create_vector_from_slice() {
        let _: Vector<i32, 3> = Vector::from([1, 2, 3]);
    }

    #[test]
    fn vector_eq() {
        let vec1: Vector<i32, 3> = Vector::from([1, 2, 3]);
        let vec2: Vector<i32, 3> = Vector::from([1, 2, 3]);
        assert_eq!(vec1, vec2);
    }

    #[test]
    fn add_vectors_same_type() {
        let vec1: Vector<i32, 3> = Vector::from([1, 2, 3]);
        let vec2: Vector<i32, 3> = Vector::from([4, 5, 6]);
        let vec_ans: Vector<i32, 3> = Vector::from([5, 7, 9]);
        assert_eq!(vec1.clone() + vec2.clone(), vec_ans);
        assert_eq!(vec2 + vec1, vec_ans);
    }

    #[test]
    fn vector_debug() {
        let vec1: Vector<i32, 3> = Vector::from([1, 2, 3]);
        assert_eq!(format!("{:?}", vec1), "Vector<i32,3> { items: [1, 2, 3] }");
    }

    #[test]
    fn scalar_mult_same_type() {
        let vec1a: Vector<i32, 3> = Vector::from([1, 2, 3]);
        let vec1b: Vector<i32, 3> = Vector::from([1, 2, 3]);
        let vec2: Vector<i32, 3> = Vector::from([2, 4, 6]);
        assert_eq!(vec2, vec1a * i32::from(2));
        assert_eq!(vec2, i32::from(2) * vec1b);
    }

    #[test]
    fn dot_product() {
        let vec1: Vector<i32, 3> = Vector::from([1, 2, 3]);
        let vec2: Vector<i32, 3> = Vector::from([4, 5, 6]);
        assert_eq!(32, vec1.clone().dot(vec2.clone()));
        assert_eq!(32, vec2.dot(vec1));
    }
}

#[cfg(test)]
#[macro_use]
mod matrix_tests {
    use crate::mat;
    use crate::matrix::*;
    use crate::vector::*;

    #[test]
    fn create_matrix_new() {
        let _: Matrix<i32, 2, 2> = Matrix::new();
    }

    #[test]
    fn create_matrix_from_slice_of_slices() {
        let _: Matrix<i32, 2, 2> = Matrix::from([[1, 2], [3, 4]]);
    }

    #[test]
    fn matrix_debug() {
        let m1: Matrix<i32, 2, 2> = Matrix::from([[1, 2], [3, 4]]);
        assert_eq!(
            format!("{:?}", m1),
            "Matrix<i32,2,2>\n1 3 \n2 4 \n"
        );
    }

    #[test]
    fn matrix_add() {
        let m1: Matrix<i32, 2, 2> = Matrix::from([[1, 2], [3, 4]]);
        let m2: Matrix<i32, 2, 2> = Matrix::from([[1, 1], [1, 1]]);
        let m_ans: Matrix<i32, 2, 2> = Matrix::from([[2, 3], [4, 5]]);
        assert_eq!(m_ans, m1 + m2);
        assert_eq!(m_ans, m2 + m1);
    }

    #[test]
    fn scalar_mult_same_type() {
        let mat1: Matrix<i32, 2, 2> = Matrix::from([[1, 2], [3, 4]]);
        let mat2: Matrix<i32, 2, 2> = Matrix::from([[1, 2], [3, 4]]);
        let mat_ans: Matrix<i32, 2, 2> = Matrix::from([[3, 6], [9, 12]]);
        assert_eq!(mat_ans, mat1 * i32::from(3));
        assert_eq!(mat_ans, i32::from(3) * mat2);
    }

    #[test]
    fn matrix_get() {
        let mat1: Matrix<i32, 2, 2> = Matrix::from([[1, 2], [3, 4]]);
        assert_eq!(mat1.get(0, 0).unwrap(), i32::from(1));
        assert_eq!(mat1.get(1, 0).unwrap(), i32::from(2));
        assert_eq!(mat1.get(0, 1).unwrap(), i32::from(3));
        assert_eq!(mat1.get(1, 1).unwrap(), i32::from(4));
    }

    #[test]
    fn matrix_get_transpose() {
        let mat1: Matrix<i32, 2, 2> = Matrix::from([[1, 2], [3, 4]]);
        let mat_ans: Matrix<i32, 2, 2> = Matrix::from([[1, 3], [2, 4]]);
        assert_eq!(mat_ans, mat1.get_transpose());
    }

    #[test]
    fn matrix_get_cols() {
        let mat1: Matrix<i32, 2, 2> = Matrix::from([[1, 2], [3, 4]]);
        let col0: Vector<i32, 2> = Vector::from([1, 2]);
        let col1: Vector<i32, 2> = Vector::from([3, 4]);
        assert_eq!(col0, mat1.get_cols().nth(0).unwrap());
        assert_eq!(col1, mat1.get_cols().nth(1).unwrap());
    }

    #[test]
    fn matrix_get_rows() {
        let mat1: Matrix<i32, 2, 2> = Matrix::from([[1, 3], [2, 4]]);
        let row0: Vector<i32, 2> = Vector::from([1, 2]);
        let row1: Vector<i32, 2> = Vector::from([3, 4]);
        assert_eq!(row0, mat1.get_rows().nth(0).unwrap());
        assert_eq!(row1, mat1.get_rows().nth(1).unwrap());
    }

    #[test]
    fn matrix_get_col() {
        let mat1: Matrix<i32, 2, 2> = Matrix::from([[1, 2], [3, 4]]);
        let row0: Vector<i32, 2> = Vector::from([1, 2]);
        let row1: Vector<i32, 2> = Vector::from([3, 4]);
        assert_eq!(row0, mat1.get_col(0).unwrap());
        assert_eq!(row1, mat1.get_col(1).unwrap());
    }

    #[test]
    fn matrix_get_row() {
        let mat1: Matrix<i32, 2, 2> = Matrix::from([[1, 2], [3, 4]]);
        let col0: Vector<i32, 2> = Vector::from([1, 3]);
        let col1: Vector<i32, 2> = Vector::from([2, 4]);
        assert_eq!(col0, mat1.get_row(0).unwrap());
        assert_eq!(col1, mat1.get_row(1).unwrap());
    }

    #[test]
    fn matrix_mul() {
        /*
         * 1 3 * 5 7 =
         * 2 4   6 8
         */
        let mat1: Matrix<i32, 2, 2> = Matrix::from([[1, 2], [3, 4]]);
        let mat2: Matrix<i32, 2, 2> = Matrix::from([[5, 6], [7, 8]]);
        let mat_ans: Matrix<i32, 2, 2> = Matrix::from([[23, 34], [31, 46]]);
        assert_eq!(mat_ans, mat1 * mat2);
    }

    #[test]
    fn matrix_identity() {
        let mat1: Matrix<i32, 2, 2> = Matrix::from([[1, 0], [0, 1]]);
        assert_eq!(mat1, Matrix::<i32, 2, 2>::identity());
    }

    #[test]
    fn matrix_identity_mul() {
        let mat1: Matrix<i32, 2, 2> = Matrix::from([[1, 2], [3, 4]]);
        let mat2: Matrix<i32, 2, 2> = Matrix::<i32, 2, 2>::identity();
        let mat_ans: Matrix<i32, 2, 2> = Matrix::from([[1, 2], [3, 4]]);
        assert_eq!(mat_ans, mat1 * mat2);
        assert_eq!(mat_ans, mat2 * mat1);
    }

    #[test]
    fn matrix_row_swap() {
        let mut mat1: Matrix<i32, 2, 2> = Matrix::from([[1, 2], [3, 4]]);
        let mat_ans: Matrix<i32, 2, 2> = Matrix::from([[2, 1], [4, 3]]);
        mat1.row_swap(0, 1);
        assert_eq!(mat_ans, mat1);
    }

    #[test]
    #[should_panic]
    fn matrix_row_swap_out_of_bounds() {
        let mut mat1: Matrix<i32, 2, 2> = Matrix::from([[1, 2], [3, 4]]);
        mat1.row_swap(0, 2)
    }

    #[test]
    fn matrix_row_mul() {
        let mut mat1: Matrix<i32, 2, 2> = Matrix::from([[1, 2], [3, 4]]);
        let mat_ans: Matrix<i32, 2, 2> = Matrix::from([[2, 2], [6, 4]]);
        mat1.row_mult(0, 2);
        assert_eq!(mat_ans, mat1);
    }

    #[test]
    #[should_panic]
    fn matrix_row_mul_out_of_bounds() {
        let mut mat1: Matrix<i32, 2, 2> = Matrix::from([[1, 2], [3, 4]]);
        mat1.row_mult(3, 3);
    }

    #[test]
    fn matrix_row_add() {
        let mut mat1: Matrix<i32, 2, 2> = Matrix::from([[1, 2], [3, 4]]);
        let mat_ans: Matrix<i32, 2, 2> = Matrix::from([[5, 2], [11, 4]]);
        mat1.row_add(0, 1, 2);
        assert_eq!(mat_ans, mat1);
    }

    #[test]
    #[should_panic]
    fn matrix_row_add_out_of_bounds() {
        let mut mat1: Matrix<i32, 2, 2> = Matrix::from([[1, 2], [3, 4]]);
        mat1.row_add(3, 1, 3);
    }

    #[test]
    fn matrix_macro_test() {
        let mat1: Matrix<i32, 2, 3> = mat![1, 2, 3; 4, 5, 6];
        let mat_ans: Matrix<i32, 2, 3> = Matrix::from([[1, 4], [2, 5], [3, 6]]);
        assert_eq!(mat_ans, mat1);
    }

    #[test]
    fn matrix_inverse() {
        let mat1: Matrix<f64, 2, 2> = mat![1.0, 2.0; 3.0, 4.0];
        let mat_ans: Matrix<f64, 2, 2> = (-1.0 / 2.0) * mat![4.0, -2.0; -3.0, 1.0];
        assert_eq!(mat_ans, mat1.inverse().unwrap());
    }
}

#[cfg(test)]
mod augmented_matrix_tests {
    use crate::matrix::*;
    use crate::mat;

    #[test]
    fn aug_matrix_from() {
        let mat1: Matrix<i32, 2, 3> = mat![1, 2, 3; 4, 5, 6];
        let mat2: Matrix<i32, 2, 4> = mat![1, 2, 3, 4; 5, 6, 7, 8];
        let _aug_mat: AugmentedMatrix<i32, 2, 3, 4> = AugmentedMatrix::from((mat1, mat2));
    }

    #[test]
    fn aug_matrix_row_swap() {
        let mut aug_mat: AugmentedMatrix<i32, 2, 4, 3> = AugmentedMatrix::from((
            mat![1, 2, 3, 4; 5, 6, 7, 8],
            mat![2, 3, 4; 5, 6, 7],
        ));
        aug_mat.row_swap(0, 1);
        let aug_ans: AugmentedMatrix<i32, 2, 4, 3> = AugmentedMatrix::from((
            mat![5, 6, 7, 8; 1, 2, 3, 4],
            mat![5, 6, 7; 2, 3, 4],
        ));
        assert_eq!(aug_ans, aug_mat);
    }

    #[test]
    fn aug_matrix_row_add() {
        let mut aug_mat: AugmentedMatrix<i32, 2, 4, 3> = AugmentedMatrix::from((
            mat![1, 2, 3, 4; 5, 6, 7, 8],
            mat![2, 3, 4; 5, 6, 7],
        ));
        aug_mat.row_add(0, 1, 2);
        let aug_ans: AugmentedMatrix<i32, 2, 4, 3> = AugmentedMatrix::from((
            mat![11, 14, 17, 20; 5, 6, 7, 8],
            mat![12, 15, 18; 5, 6, 7],
        ));
        assert_eq!(aug_ans, aug_mat);
    }

    #[test]
    fn aug_matrix_row_mult() {
        let mut aug_mat: AugmentedMatrix<i32, 2, 4, 3> = AugmentedMatrix::from((
            mat![1, 2, 3, 4; 5, 6, 7, 8],
            mat![2, 3, 4; 5, 6, 7],
        ));
        aug_mat.row_mult(0, 2);
        let aug_ans: AugmentedMatrix<i32, 2, 4, 3> = AugmentedMatrix::from((
            mat![2, 4, 6, 8; 5, 6, 7, 8],
            mat![4, 6, 8; 5, 6, 7],
        ));
        assert_eq!(aug_ans, aug_mat);
    }

    #[test]
    fn aug_matrix_reduce_left_to_id() {
        let mut aug_mat: AugmentedMatrix<f32, 2, 2, 2> = AugmentedMatrix::from((
            mat![1_f32, 2_f32; 3_f32, 4_f32],
            Matrix::<f32, 2, 2>::identity() ));
        aug_mat.reduce_left();
        let aug_reduced: AugmentedMatrix<f32, 2, 2, 2> = AugmentedMatrix::from((
            Matrix::<f32, 2, 2>::identity(),
            (1_f32 / -2_f32) * mat![4_f32, -2_f32; -3_f32, 1_f32] ));
        assert_eq!(aug_reduced, aug_mat);
    }

}
