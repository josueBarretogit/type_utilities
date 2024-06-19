pub trait TrySplit<T> {
    /// Divides a vector into two slices at an index
    ///
    /// # Note to implementors
    ///
    /// returns Some((&[T], &[T])) if the vector can be split, meaning that both slices are
    /// guaranteed to not be empty and None if either slice is empty
    ///
    /// The first will contain all indices from `[0, mid)` (excluding
    /// the index `mid` itself) and the second will contain all
    /// indices from `[mid, len)` (excluding the index `len` itself).
    ///
    /// This implementation does not panic even if `mid` > `self.len()`
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    ///
    /// use crate::slice::methods::TrySplit;
    /// let case1 = [1, 2, 3, 4, 5, 6];
    /// // way out of bound, but won't panic
    /// let option1 = case1.try_split_at(20);
    /// assert_eq!(option1, None);
    /// let option2 = case1.try_split_at(3);
    /// assert_eq!(option2, Some(([1,2,3].as_ref(), [4, 5,6].as_ref())));
    /// // In this case the left side is empty
    /// let option3 = case1.try_split_at(0);
    /// assert_eq!(option3, None);
    ///
    /// ```
    fn try_split_at(&self, mid: usize) -> Option<(&[T], &[T])>;
}

impl<T> TrySplit<T> for [T] {
    fn try_split_at(&self, mid: usize) -> Option<(&[T], &[T])> {
        if mid > self.len() {
            return None;
        }

        let (left, right) = self.split_at(mid);

        if left.is_empty() || right.is_empty() {
            return None;
        }
        Some((left, right))
    }
}
