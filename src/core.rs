use std::cmp::Ordering;
use std::mem::{MaybeUninit};
use crate::fixed_size_min_heap::FixedSizeMinHeap;

#[inline]
pub fn max_n_by_from_iter<const K: usize, T>(mut iter: impl Iterator<Item=T>, f: impl FnMut(&T, &T) -> Ordering) -> [T; K] {
    // Create empty data & initialize
    // Safety: Creating a fully uninitialized array is safe
    let mut data: [MaybeUninit<T>; K] = unsafe { MaybeUninit::uninit().assume_init() };
    for i in 0..K {
        if let Some(v) = iter.next() {
            // Safety: Now 0..=i is initialized
            data[i] = MaybeUninit::new(v);
        } else {
            assert!(false, "Iterator needs at least K elements")
        }
    }

    // Safety: At the end, 0..K is initialized which is the entire array. We can assume it it initialized.
    let mut data: [T; K] = unsafe { data.map(|e| e.assume_init()) };
    max_n_by_with_slice(&mut data, iter, f);
    data
}

#[inline]
pub fn max_n_by_with_slice<const K: usize, T>(data: &mut [T; K], mut iter: impl Iterator<Item=T>, f: impl FnMut(&T, &T) -> Ordering) {
    let mut heap = FixedSizeMinHeap::from_array(data, f);
    while let Some(v) = iter.next() {
        heap.push(v);
    }
}

#[cfg(test)]
mod tests {
    use rand::{Rng, SeedableRng, RngCore};
    use super::max_n_by_from_iter;

    #[inline]
    fn max_n<const K: usize, T: Ord>(iter: impl Iterator<Item=T>) -> [T; K] {
        max_n_by_from_iter(iter, T::cmp)
    }

    #[test]
    fn max_n1() {
        let a = [1];
        let b = [1];
        assert_eq!(max_n(a.into_iter()), b);
    }

    #[test]
    fn max_n2() {
        let a = [1,2];
        let b = [2];
        assert_eq!(max_n(a.into_iter()), b);
    }

    #[test]
    fn max_n3() {
        let a = [1,2,3];
        let b = [3];
        assert_eq!(max_n(a.into_iter()), b);
    }

    #[test]
    fn max_n3_2() {
        let a = [1,2,3];
        let b = [2,3];
        assert_eq!(max_n(a.into_iter()), b);
    }

    #[test]
    fn smoke_largerange() {
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(42);

        for _ in 0..1000 {
            let mut a = Vec::new();
            for _ in 0..(1000 + rng.gen_range(0..1000)) {
                a.push(rng.next_u64());
            }
            let mut b: [u64; 10] = max_n(a.iter().map(|x| *x));
            a.sort();
            b.sort();

            assert_eq!(a[a.len() - 10..], b);
        }
    }

    #[test]
    fn smoke_smallrange() {
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(42);

        for _ in 0..1000 {
            let mut a = Vec::new();
            for _ in 0..(1000 + rng.gen_range(0..1000)) {
                a.push(rng.gen_range(0..16));
            }
            let mut b: [u64; 10] = max_n(a.iter().map(|x| *x));
            a.sort();
            b.sort();

            assert_eq!(a[a.len() - 10..], b);
        }
    }
}

