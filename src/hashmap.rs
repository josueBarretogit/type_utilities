//! This modules contains traits that define new methods for `Hashmap<T, V>`

use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};
pub trait SetOperation<K, V, S> {
    /// Returns the elements representing the union,
    /// i.e., all the elements that are in `self` and `other` without duplicates
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::hashmap::SetOperation;
    ///
    /// let a = HashMap::from([("a", 2), ("b", 1), ("d", 3)]);
    /// let b = HashMap::from([("a", 2), ("b", 1), ("c", 4)]);
    ///
    /// let union_hashmap = a.union(&b);
    ///
    /// assert_eq!(
    ///  HashMap::from([("a", 2), ("b", 1), ("c", 4), ("d", 3)]),
    ///  union_hashmap
    /// );
    ///
    /// let c = HashMap::from([("a", 2), ("b", 1), ("c", 3)]);
    /// let d = HashMap::from([("d", 2), ("f", 1), ("d", 3)]);
    ///
    /// let union_hashmap2 = c.union(&d);
    ///
    /// assert_eq!(
    ///  HashMap::from([("a", 2), ("b", 1), ("c", 3), ("d", 2), ("f", 1), ("d", 3)]),
    ///  union_hashmap2
    /// );
    ///
    /// ```
    fn union(&self, other: &HashMap<K, V>) -> HashMap<K, V>
    where
        K: Eq + PartialEq + Hash + Clone,
        V: Eq + PartialEq + Clone,
        S: BuildHasher;
}

impl<K, V, S> SetOperation<K, V, S> for HashMap<K, V, S> {
    fn union(&self, other: &HashMap<K, V>) -> HashMap<K, V>
    where
        K: Eq + PartialEq + Hash + Clone,
        V: Eq + PartialEq + Clone,
        S: BuildHasher,
    {
        let mut union_result: HashMap<K, V> = HashMap::new();

        for (key, value) in self {
            union_result.insert(key.clone(), value.clone());
        }

        for (key, value) in other {
            union_result.entry(key.clone()).or_insert(value.clone());
        }

        union_result
    }
}

#[cfg(test)]
mod test {
    use std::collections::{btree_set, HashMap};

    #[test]
    fn union_works() {
        use crate::hashmap::SetOperation;

        let a = HashMap::from([("a", 2), ("b", 1), ("d", 3)]);
        let b = HashMap::from([("a", 2), ("b", 1), ("c", 4)]);

        let union_hashmap = a.union(&b);

        assert_eq!(
            HashMap::from([("a", 2), ("b", 1), ("c", 4), ("d", 3)]),
            union_hashmap
        );

        let c = HashMap::from([("a", 2), ("b", 1), ("c", 3) , ("d", 2)]);
        let d = HashMap::from([("d", 2), ("f", 1), ("d", 3)]);

        let union_hashmap2 = c.union(&d);

        assert_eq!(
            HashMap::from([("a", 2), ("b", 1), ("c", 3), ("d", 2), ("f", 1), ("d", 3)]),
            union_hashmap2
        );
    }
}
