//! A experimental library providing a compile-time length-checking vector implementation.
//!
//! See [Vector](struct.Vector.html) for basic details.

extern crate typenum;

use std::fmt;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

use typenum::{True, IsLess, Unsigned};

pub use typenum::consts as length;
pub mod index;

/// A vector with compile-time length checking.
///
/// # Examples
///
/// Basic creation and indexing:
/// ```
/// use vector::{Vector, index::*, length::*};
/// let v = Vector::<_, U3>::from(vec![1, 3, 4]);
/// assert_eq!(v.len(), 3);
/// assert_eq!(v[_0], 1);
/// assert_eq!(v[_1], 3);
/// assert_eq!(v[_2], 4);
/// ```
/// This creates a `Vector` with length 3 (specified by the type parameter `U3` from the `length`
/// module) from a `Vec`. Indexing into a `Vector` uses the special consts (from the `index` module)
/// `_0`, `_1`, `_2`, ....
///
/// Macro-based creation:
/// ```
/// #[macro_use] extern crate vector;
/// use vector::{Vector, index::*, length::*};
/// fn main() {
///     let v = vector![1, 3, 4];
///     assert_eq!(v.len(), 3);
///     assert_eq!(v[_0], 1);
///     assert_eq!(v[_1], 3);
///     assert_eq!(v[_2], 4);
/// }
/// ```
///
/// Compile-time bounds checking:
/// ```compile_fail
/// #[macro_use] extern crate vector;
/// use vector::{Vector, index::*, length::*};
/// fn main() {
///     let v = vector![1, 3, 4];
///     assert_eq!(v[_3], 1); // doesn't compile!!!
/// }
/// ```
#[derive(Clone)]
pub struct Vector<T, L> {
    inner: Vec<T>,
    length: PhantomData<L>,
}

impl<T, L> Vector<T, L> {
    /// Returns the length of this `Vector`.
    pub fn len(&self) -> usize
    where
        L: Unsigned
    {
        L::to_usize()
    }

    /// Creates a `Vector` of length `L` from a repeated element.
    pub fn from_elem(elem: T) -> Vector<T, L>
    where
        T: Clone,
        L: Unsigned
    {
        from_elem::<L, T>(elem)
    }
}

impl<T, L> From<Vec<T>> for Vector<T, L> {
    fn from(orig: Vec<T>) -> Vector<T, L> {
        Vector {
            inner: orig,
            length: PhantomData,
        }
    }
}

/// Creates a `Vector` of length `L` from a repeated element.
pub fn from_elem<L: Unsigned, T: Clone>(elem: T) -> Vector<T, L> {
    Vector {
        inner: vec![elem; L::USIZE],
        length: PhantomData,
    }
}

impl<T, L> fmt::Debug for Vector<T, L>
where
    T: fmt::Debug,
    L: Unsigned
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vector {{ inner: {:?}, length: {} }}", self.inner, L::USIZE)
    }
}

impl<I, T, L> Index<I> for Vector<T, L>
where
    Vec<T>: Index<usize>,
    L: Unsigned,
    I: Unsigned + IsLess<L, Output=True>,
{
    type Output = <Vec<T> as Index<usize>>::Output;

    fn index(&self, _: I) -> &Self::Output {
        &self.inner[I::to_usize()]
    }
}

impl<I, T, L> IndexMut<I> for Vector<T,L>
where
    Vec<T>: IndexMut<usize>,
    L: Unsigned,
    I: Unsigned + IsLess<L, Output=True>
{
    fn index_mut(&mut self, _: I) -> &mut Self::Output {
        &mut self.inner[I::to_usize()]
    }
}

/// Utility macro for counting number of expressions and returning a `typenum` value. Used by
/// the [vector](macro.vector.html) macro.
#[macro_export]
macro_rules! count_expressions {
    ($last:expr) => (typenum::consts::U1);
    ($head:expr, $($tail:expr),*) => (
        typenum::operator_aliases::Add1<count_expressions![$($tail),*]>
    )
}

/// `Vector` creation macro. See [Vector](struct.Vector.html) for an example.
#[macro_export]
macro_rules! vector {
    ($($x:expr),*) => (
        $crate::Vector::<_, count_expressions![$($x),*]>::from(vec![$($x),*])
    );
    ($($x:expr,)*) => (vector![$($x),*]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{index::*, length::*};

    #[test]
    fn create() {
        let v = Vector::<_, U3>::from(vec![1, 3, 4]);
        assert_eq!(v.len(), 3);
        assert_eq!(v[_0], 1);
        assert_eq!(v[_1], 3);
        assert_eq!(v[_2], 4);

        let v = vector![1, 3, 4];
        assert_eq!(v.len(), 3);
        assert_eq!(v[_0], 1);
        assert_eq!(v[_1], 3);
        assert_eq!(v[_2], 4);
        // assert_eq!(v[_3], 4); // won't compile

        let v = from_elem::<U3, _>(1);
        assert_eq!(v.len(), 3);
        assert_eq!(v[_0], 1);
        assert_eq!(v[_1], 1);
        assert_eq!(v[_2], 1);
    }
}
