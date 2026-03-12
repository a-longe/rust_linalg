use std::{ops::Add, ops::Sub, ops::Mul};
use std::fmt::Debug;
use std::any;

#[derive(Clone)]
pub struct Vector<T: VectorItem, const C: usize> {
    items: [T; C]
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

impl<T: VectorItem, const C: usize> Vector<T, C> {
    pub fn new() -> Vector<T, C> {
        return Vector { items: [T::default(); C] };
    }
}

impl<T: VectorItem, const C: usize> Add<Vector<T, C>> for Vector<T, C> {
    type Output = Vector<T, C>;
    fn add(self, rhs: Vector<T, C>) -> Self::Output {
        let mut new_s: [T; C] = [T::default(); C];
        for i in 0..C {
            new_s[i] = self.items[i] + rhs.items[i];
        }
        Vector{ items: new_s }
    }
}

impl<T: VectorItem, const C:usize> From<[T; C]> for Vector<T, C> {
    fn from(value: [T; C]) -> Self {
        Vector { items: value }
    }
}

impl<T: VectorItem, const C: usize> PartialEq<Vector<T, C>> for Vector<T, C> {
    fn eq(&self, other: &Vector<T, C>) -> bool {
        self.items == other.items
    }
    fn ne(&self, other: &Vector<T, C>) -> bool {
        self.items != other.items
    }
}

impl<T: VectorItem, const C: usize> Debug for Vector<T, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("Vector<{},{}>", any::type_name::<T>(), C))
            .field("items", &self.items)
            .finish()
    }
}


// Macro to implement scalar multiplication
// Macro will be called on a list of types we consider 'scalar'

macro_rules! impl_scalar_mul {
    ($($t:ty),*) => {
        $(
            impl<const C: usize> Mul<Vector<$t, C>> for $t {
                type Output = Vector<$t, C>;

                fn mul(self, rhs: Vector<$t, C>) -> Self::Output {
                    let mut new: [$t; C] = [<$t>::default(); C];
                    for i in 0..C {
                        new[i] = self * rhs.items[i];
                    }
                    Vector { items:new }
                }
            }
            impl<const C: usize> Mul<$t> for Vector<$t, C> {
                type Output = Vector<$t, C>;

                fn mul(self, rhs: $t) -> Self::Output {
                    rhs * self
                }
            }
        )
*};}
impl_scalar_mul!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);

impl<T: VectorItem, const C:usize> Vector<T, C> {
    pub fn dot(self, rhs: Vector<T, C>) -> T {
        let mut sum: T = T::default();
        for i in 0..C {
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
