pub trait Toggle {
    /// Inverts the value of the variable, if it is true then self will become false and viceversa
    /// 
    /// # Examples
    /// ```no_run
    /// let mut case1 = true;
    ///
    /// case1 = !case;
    ///
    /// //instead call: 
    /// case1.toggle()
    /// ```
    /// Similar to `vec.is_empty()` this could be considered a more idiomatic way of
    /// inverting the value of a boolean
    fn toggle(&mut self);
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
