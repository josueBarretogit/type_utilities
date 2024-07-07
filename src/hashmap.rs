//! This modules contains traits that define new methods for `Hashmap<T, V>`

use std::collections::HashMap;
use std::hash::Hasher;
pub trait SetOperation<K, V> {
    /// Returns the elements representing the union,
    /// i.e., all the elements that are in self and other without duplicates `self` and `other`,
    /// in ascending order.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeSet;
    ///
    /// let mut a = BTreeSet::new();
    /// a.insert(1);
    /// a.insert(2);
    ///
    /// let mut b = BTreeSet::new();
    /// b.insert(2);
    /// b.insert(3);
    ///
    /// let intersection: Vec<_> = a.intersection(&b).cloned().collect();
    /// assert_eq!(intersection, [2]);
    /// ```
    fn union(&self, other: HashMap<K, V>) -> HashMap<K, V>
    where
        K: Eq;
}

impl<K, V, S> SetOperation<K, V> for HashMap<K, V, S> {
    fn union(&self, other:  HashMap<K, V>) -> HashMap<K, V>
    where
        K: Eq,
    {
        other
    }
}

#[cfg(test)]
mod test {
    use std::collections::{btree_set, HashMap};

    #[test]
    fn union_works() {
        use crate::hashmap::SetOperation;

        let hash1 = HashMap::from([("a", 2), ("b", 1), ("d", 3)]);
        let hash2 = HashMap::from([("a", 2), ("c", 4), ("b", 1)]);

        assert_eq!(HashMap::from([("a", 2), ("b", 1), ("c", 4), ("d", 3)]), hash1.union(hash2));
    }
}
