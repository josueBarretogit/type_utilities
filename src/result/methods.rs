pub trait Filter<T, E> {
    /// This method returns:
    ///
    /// `Ok(&t)` if Result is the `Ok` variant and `predicate` returns `true`
    ///
    /// #  Errors
    ///
    /// This method returns:
    ///
    /// `Err(None)` if `Result` is the `Ok` variant and `predicate` returns `false`
    /// `Err(Some(&e))` if `Result` is the `Err` variant
    ///
    /// # Examples
    /// ```rust
    ///
    /// use crate::result::methods::Filter;
    /// let case1 = "2".parse::<i32>();
    /// let case1 = binding.filter(|nu| *nu > 1);
    /// assert_eq!(case1, Ok(&2));
    ///
    /// let case2 = "err".parse::<i32>();
    /// let case2 = case2.filter(|nu| *nu < 10);
    ///
    /// match case2 {
    ///     Ok(_) => {},
    ///     Err(e) => {
    ///         assert!(e.is_some());
    ///     },
    /// };
    ///
    /// let case3 = "10".parse::<i32>();
    /// let case3 = case3.filter(|nu| *nu > 20);
    ///
    /// match case3 {
    ///     Ok(_) => {},
    ///     Err(e) => assert!(e.is_none())
    /// };
    ///
    /// ```
    fn filter<F: FnOnce(&T) -> bool>(&self, predicate: F) -> Result<&T, Option<&E>>;
}

impl<T, E> Filter<T, E> for Result<T, E> {
    fn filter<F: FnOnce(&T) -> bool>(&self, predicate: F) -> Result<&T, Option<&E>> {
        match self {
            Ok(ok) => {
                if predicate(ok) {
                    return Ok(ok);
                }
                Err(None)
            }
            Err(e) => Err(Some(e)),
        }
    }
}
