use std::cmp::Ordering;
use std::cmp::Ordering::{Greater, Less};
use std::mem;

pub(crate) struct FixedSizeMinHeap<'a, const K: usize, T, F: FnMut(&T, &T) -> Ordering> {
    data: &'a mut [T; K],
    f: F,
}

impl<'a, const K: usize, T, F: FnMut(&T, &T) -> Ordering> FixedSizeMinHeap<'a, K, T, F> {
    pub(crate) fn from_array(data: &'a mut [T; K], f: F) -> Self {
        let mut heap = FixedSizeMinHeap { data, f };
        heap.build();
        heap
    }

    #[inline]
    fn build(&mut self) {
        for k in (0..K/2).rev() {
            self.sift_down(k);
        }
    }

    #[inline]
    fn sift_down(&mut self, mut m: usize) {
        loop {
            if m >= K { return; }

            let mut n = m;

            let l = 2*m + 1;
            let r = 2*m + 2;

            if l < K && (self.f)(&self.data[l], &self.data[n]) == Less {
                n = l;
            }
            if r < K && (self.f)(&self.data[r], &self.data[n]) == Less {
                n = r;
            }

            if m == n { return };

            self.data.swap(m, n);
            m = n;
        }
    }

    #[inline]
    pub(crate) fn push(&mut self, mut t: T) {
        if (self.f)(&t, &self.data[0]) == Greater {
            mem::swap(&mut t, &mut self.data[0]);
            self.sift_down(0);
        }
    }
}