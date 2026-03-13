pub mod vector;
pub mod matrix;

#[cfg(test)]
mod vector_tests {
    use crate::vector::*;

    #[test]
    fn create_vector_new() {
        let _: Vector::<u32, 5> = Vector::new();
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
        assert_eq!(vec2, vec1a*i32::from(2));
        assert_eq!(vec2, i32::from(2)*vec1b);
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
mod matrix_tests {
    use crate::matrix::*;

    #[test]
    fn create_matrix_new() {
        let _: Matrix<i32, 2, 2> = Matrix::new();
    }

    #[test]
    fn create_matrix_from_slice_of_slices() {
        let _: Matrix<i32, 2, 2> = Matrix::from([[1, 2], [3, 4]]);
    }

    #[test]
    fn vector_debug() {
        let m1: Matrix<i32, 2, 2> = Matrix::from([[1, 2], [3, 4]]);
        assert_eq!(format!("{:?}", m1), "Matrix<i32,2,2> { items: [Vector<i32,2> { items: [1, 2] }, Vector<i32,2> { items: [3, 4] }] }");
    }

    #[test]
    fn matrix_add() {
        let m1: Matrix<i32, 2, 2> = Matrix::from([[1, 2], [3, 4]]);
        let m2: Matrix<i32, 2, 2> = Matrix::from([[1, 1], [1, 1]]);
        let m_ans: Matrix<i32, 2, 2> = Matrix::from([[2, 3], [4, 5]]);
        assert_eq!(m_ans, m1+m2);
        assert_eq!(m_ans, m2+m1);
    }

    #[test]
    fn scalar_mult_same_type() {
        let mat1: Matrix<i32, 2, 2> = Matrix::from([[1, 2], [3, 4]]);
        let mat2: Matrix<i32, 2, 2> = Matrix::from([[1, 2], [3, 4]]);
        let mat_ans: Matrix<i32, 2, 2> = Matrix::from([[3, 6], [9, 12]]);
        assert_eq!(mat_ans, mat1*i32::from(3));
        assert_eq!(mat_ans, i32::from(3)*mat2);
    }
}
