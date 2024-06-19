pub trait ReplaceVec<F, V>
where
    F: FnMut(&V) -> bool,
    V: PartialEq + Clone,
{
    /// Creates a new vector with an element replaced
    ///
    /// # Considerations
    ///
    /// if an element is found the methods return a vector with that element replaced
    /// if it is not found then the original vector will be returned
    ///
    /// # Examples
    ///
    /// ```no_run
    ///
    /// use crate::vec::methods::*;
    /// #[derive(Debug, Default, Clone, PartialEq)]
    /// struct Example {
    ///     name: String,
    ///     id: i32,
    ///     other_property: Vec<u8>,
    /// }
    ///
    ///     let case1 = vec![1, 3, 4, 5];
    ///
    ///     let replaced_vec = case1.replace(|item| *item == 3, 4);
    ///
    ///     assert_eq!(vec![1, 4, 4, 5], replaced_vec);
    ///
    ///     let replaced_vec2 = case1.replace(|item| *item == 5, 10);
    ///
    ///     assert_eq!(vec![1, 3, 4, 10], replaced_vec2);
    ///
    ///     let case2: Vec<Example> = vec![
    ///         Example {
    ///             name: "josh".to_string(),
    ///             id: 1,
    ///             other_property: vec![],
    ///         },
    ///         Example {
    ///             name: "ua".to_string(),
    ///             id: 2,
    ///             other_property: vec![],
    ///         },
    ///     ];
    ///
    ///     let replaced_struct_vec = case2.replace(
    ///         |example| example.name.as_str() == "josh",
    ///         Example {
    ///             name: "replaced".to_string(),
    ///             id: 3,
    ///             other_property: vec![1, 2],
    ///         },
    ///     );
    ///
    ///     assert_eq!(
    ///         vec![
    ///             Example {
    ///                 name: "replaced".to_string(),
    ///                 id: 3,
    ///                 other_property: vec![1, 2],
    ///             },
    ///             Example {
    ///                 name: "ua".to_string(),
    ///                 id: 2,
    ///                 other_property: vec![],
    ///             },
    ///         ],
    ///         replaced_struct_vec
    ///     );
    ///
    /// ```
    fn replace(&self, searcher: F, value: V) -> Vec<V>;
    /// Replaces an element of the vector (mutates the vector)
    ///
    /// # Examples
    ///
    ///```no_run
    /// let mut case1 = vec![1, 2, 3, 4, 5];
    /// // replace the number 2
    /// case1.replace_mut(|item| *item == 2, 3);
    /// assert_eq!(vec![1, 3, 3, 4, 5], case1);
    /// let mut case2_struct_case = vec![
    ///     Example {
    ///         name: "name1".to_string(),
    ///         id: 1,
    ///         other_property: vec![1, 2, 3],
    ///     },
    ///     Example {
    ///         name: "other".to_string(),
    ///         id: 2,
    ///         other_property: vec![3, 2, 4],
    ///     },
    /// ];
    /// case2_struct_case.replace_mut(
    ///     |item| item.name.as_str() == "other",
    ///     Example {
    ///         name: "other changed".to_string(),
    ///         id: 2,
    ///         other_property: vec![3, 2, 4],
    ///     },
    /// );
    /// let case_to_compare_struct = vec![
    ///     Example {
    ///         name: "name1".to_string(),
    ///         id: 1,
    ///         other_property: vec![1, 2, 3],
    ///     },
    ///     Example {
    ///         name: "other changed".to_string(),
    ///         id: 2,
    ///         other_property: vec![3, 2, 4],
    ///     },
    /// ];
    /// assert_eq!(case_to_compare_struct, case2_struct_case);
    ///
    ///
    ///```
    fn replace_mut(&mut self, searcher: F, value: V);
}

impl<F, V> ReplaceVec<F, V> for Vec<V>
where
    F: FnMut(&V) -> bool,
    V: PartialEq + Clone,
{
    fn replace(&self, searcher: F, value: V) -> Vec<V>
    where
        F: FnMut(&V) -> bool,
        V: PartialEq + Clone,
    {
        let item = self.iter().position(searcher);
        match item {
            Some(index) => {
                let mut new_vec: Vec<V> = vec![];
                for item in self {
                    if *item == self[index] {
                        new_vec.push(value.clone());
                    } else {
                        new_vec.push(item.clone());
                    }
                }
                new_vec
            }
            None => self.clone(),
        }
    }

    fn replace_mut(&mut self, searcher: F, value: V)
    where
        F: FnMut(&V) -> bool,
        V: PartialEq + Clone,
    {
        let item_to_replace_index = self.iter().position(searcher);
        if let Some(index) = item_to_replace_index {
            self[index] = value;
        }
    }
}

pub trait TrySplit<T> {
    /// Divides a vector into two slices at an index
    ///
    /// # Note to implementors
    ///
    /// returns Some((&[T], &[T])) if the vector can be split, meaning that both sides are
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
    /// use crate::vec::methods::TrySplit;
    /// let case1 = vec![1, 2, 3, 4, 5, 6];
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

impl<T> TrySplit<T> for Vec<T> {
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
