pub trait Partition<T> {
    /// Partitions self in place so that all elements for which the `predicate`
    /// returns `true` are positioned before all elements for which it returns `false`.
    ///
    /// The function returns the index of the first element for which `predicate` returns `false`.
    fn partition<F>(&mut self, predicate: F) -> usize
    where
        F: Fn(&T) -> bool;
}

impl<T> Partition<T> for [T] {
    fn partition<F>(&mut self, predicate: F) -> usize
    where
        F: Fn(&T) -> bool,
    {
        if self.is_empty() {
            return 0;
        }

        let mut l = 0;
        let mut r = self.len() - 1;

        loop {
            while l <= r && predicate(&self[l]) {
                l += 1;
            }
            while l < r && !predicate(&self[r]) {
                r -= 1;
            }
            if l >= r {
                return l;
            }

            self.swap(l, r);
            l += 1;
            r -= 1;
        }
    }
}
