pub mod vector;

#[cfg(test)]
mod vector_tests {
    use crate::vector::*;

    #[test]
    fn create_vector_new() {
        let _: Vector::<u32, 5> = Vector::new();
    }

    #[test]
    fn create_vector_new_len_0() {
        let _: Vector::<u32, 0> = Vector::new();
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
    }

    #[test]
    fn vector_debug() {
        let vec1: Vector<i32, 3> = Vector::from([1, 2, 3]);
        assert_eq!(format!("{:?}", vec1), "Vector<i32,3> { items: [1, 2, 3] }");
    }
}
