use std::{ops::Add, ops::Sub, ops::Mul};
use std::fmt::Debug;
use std::any;

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
