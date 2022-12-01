use std::cmp::Ordering;
use crate::core::max_n_by_from_iter;

pub type MaxNIter<T, const K: usize> = std::array::IntoIter<T, K>;

///
pub trait MaxN<T>: Sized {
    fn max_n<const K: usize>(self) -> MaxNIter<T, K> where T : Ord {
        self.max_n_by(T::cmp)
    }

    fn max_n_by<const K: usize>(self, f: impl FnMut(&T, &T) -> Ordering) -> MaxNIter<T, K>;

    fn max_n_by_key<const K: usize, U: Ord>(self, mut f: impl FnMut(&T) -> U) -> MaxNIter<T, K> {
        self.max_n_by(|x, y| f(x).cmp(&f(y)))
    }

    fn min_n<const K: usize>(self) -> MaxNIter<T, K> where T : Ord {
        self.max_n_by(|x, y| x.cmp(y).reverse())
    }

    fn min_n_by<const K: usize>(self, mut f: impl FnMut(&T, &T) -> Ordering) -> MaxNIter<T, K> where T : Ord {
        self.max_n_by(|x, y| f(x, y).reverse())
    }

    fn min_n_by_key<const K: usize, U: Ord>(self, mut f: impl FnMut(&T) -> U) -> MaxNIter<T, K> {
        self.max_n_by(|x, y| f(x).cmp(&f(y)).reverse())
    }
}

impl<T, I: Iterator<Item=T>> MaxN<T> for I {
    fn max_n_by<const K: usize>(self, f: impl FnMut(&T, &T) -> Ordering) -> std::array::IntoIter<T, K> {
        max_n_by_from_iter(self.into_iter(), f).into_iter()
    }
}

