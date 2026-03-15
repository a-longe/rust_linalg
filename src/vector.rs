use std::{ops::Add, ops::Sub, ops::Mul};
use std::slice::Iter;
use std::fmt::Debug;
use std::any;

#[derive(Copy, Clone, PartialEq)]
pub struct Vector<T: VectorItem, const R: usize> {
    items: [T; R]
}

pub trait VectorItem:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + PartialOrd
    + PartialEq
    + Copy
    + Clone
    + Debug
    + Default
    + Sized
{}

// Blanket impl — any type satisfying the bounds automatically implements VectorItem
impl<T> VectorItem for T where
    T: Add<Output = T>
    + Sub<Output = T>
    + Mul<Output = T>
    + PartialOrd
    + PartialEq
    + Copy
    + Clone
    + Debug
    + Default
    + Sized
{}

impl<T: VectorItem, const R: usize> Vector<T, R> {
    pub fn new() -> Vector<T, R> {
        return Vector { items: [T::default(); R] };
    }
    pub fn get(&self, i:usize) -> Option<T> {
        if i >= R { return None; }
        Some(self.items[i])
    }
    pub fn set(&mut self, i:usize, val:T) -> bool {
        if i >= R { return false; }
        self.items[i] = val;
        true
    }
}

impl<T: VectorItem, const R: usize> Add<Vector<T, R>> for Vector<T, R> {
    type Output = Vector<T, R>;
    fn add(self, rhs: Vector<T, R>) -> Self::Output {
        let mut new_s: [T; R] = [T::default(); R];
        for i in 0..R {
            new_s[i] = self.items[i] + rhs.items[i];
        }
        Vector{ items: new_s }
    }
}

impl<T: VectorItem, const R:usize> From<[T; R]> for Vector<T, R> {
    fn from(value: [T; R]) -> Self {
        Vector { items: value }
    }
}

impl<T: VectorItem, const R: usize> Debug for Vector<T, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("Vector<{},{}>", any::type_name::<T>(), R))
            .field("items", &self.items)
            .finish()
    }
}


// Macro to implement scalar multiplication
// Macro will be called on a list of types we consider 'scalar'

macro_rules! impl_vec_scalar_mul {
    ($($t:ty),*) => {
        $(
            // right hand scalar mult
            impl<const R: usize> Mul<Vector<$t, R>> for $t {
                type Output = Vector<$t, R>;

                fn mul(self, rhs: Vector<$t, R>) -> Self::Output {
                    let mut new: [$t; R] = [<$t>::default(); R];
                    for i in 0..R {
                        new[i] = self * rhs.items[i];
                    }
                    Vector { items:new }
                }
            }
            // left hand scalar mult
            impl<const R: usize> Mul<$t> for Vector<$t, R> {
                type Output = Vector<$t, R>;

                fn mul(self, rhs: $t) -> Self::Output {
                    rhs * self
                }
            }
        )
*};}
impl_vec_scalar_mul!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);

impl<T: VectorItem, const R:usize> Vector<T, R> {
    pub fn dot(self, rhs: Vector<T, R>) -> T {
        let mut sum: T = T::default();
        for i in 0..R {
            sum = sum + (self.items[i] * rhs.items[i]);
        }
        sum
    }
    pub fn length(self) -> T {
        let mut sum = T::default();
        for val in &self.items {
            sum = sum + (*val * *val);
        }
        sum
    }
}

// TODO: add 'add' and 'mult' versions for references to vectors

impl<'a, T: VectorItem, const R:usize> IntoIterator for &'a Vector<T, R> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}
