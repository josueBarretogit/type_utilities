pub trait Toggle {
    /// Inverts the value of `bool`, if it is true then `Self` will become false and viceversa
    ///
    /// # Examples
    ///
    /// ```
    /// let mut case1 = true;
    ///
    /// case1 = !case;
    ///
    /// // instead call:
    ///
    /// case1.toggle()
    ///
    /// ```
    /// Similar to `vec.is_empty()` this could be considered a more idiomatic way of
    ///
    /// inverting the value of a boolean
    fn toggle(&mut self);
}

/// This trait declares negative or opossite, already existing methods
pub trait Not {
    /// This is the opossite of `then`
    ///
    /// Returns `Some(f())` if the `bool` is [`false`]
    /// or `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::bool::methods::Not;
    ///
    /// let case1 = false;
    /// assert_eq!(case1.not_then(|| "negative"), Some("negative"));
    ///
    /// let case2 = true;
    ///
    /// assert_eq!(case2.not_then(|| 1), None);
    /// ```
    fn not_then<T, F: FnOnce() -> T>(self, f: F) -> Option<T>;
    /// This is the opossite of `then_some`
    ///
    /// Returns `Some(t)` if the `bool` is false, otherwise return `None`
    /// # Examples
    ///
    /// ```
    /// use crate::bool::methods::Not;
    /// let case1 = false;
    ///
    /// assert_eq!(case1.not_then_some("negative"), Some("negative"));
    ///
    /// let case2 = true;
    ///
    /// assert_eq!(case2.not_then_some(1), None);
    /// ```
    fn not_then_some<T>(self, t: T) -> Option<T>;
}

#[allow(clippy::inline_always)]
impl Toggle for bool {
    #[inline(always)]
    fn toggle(&mut self) {
        match self {
            true => *self = false,
            false => *self = true,
        }
    }
}

impl Not for bool {
    #[inline]
    fn not_then<T, F: FnOnce() -> T>(self, f: F) -> Option<T> {
        if self {
            None
        } else {
            Some(f())
        }
    }

    #[inline]
    fn not_then_some<T>(self, t: T) -> Option<T> {
        if self {
            None
        } else {
            Some(t)
        }
    }
}
