pub trait IsNone {
    /// Returns true if the `option` is `None` and the closure `f` return true
    /// otherwise returns `false`
    /// # Examples
    ///
    /// ```
    /// use crate::option::methods::IsNone;
    ///
    /// let case1 = Some(1);
    ///
    /// assert!(!case1.is_none_and(|| true));
    ///
    /// let case2 : Option<i32> = None;
    ///
    /// assert!(case2.is_none_and(|| case1.is_some()));
    ///
    /// ```
    fn is_none_and<F: FnOnce() -> bool>(&self, f: F) -> bool;
}

impl<T> IsNone for Option<T> {
    fn is_none_and<F: FnOnce() -> bool>(&self, f: F) -> bool {
        match self {
            Some(_) => false,
            None => f(),
        }
    }
}
