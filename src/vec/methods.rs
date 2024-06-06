pub trait ReplaceVec<F, V>
where
    F: FnMut(&V) -> bool,
    V: PartialEq + Clone,
{
    fn replace(&self, searcher: F, value: V) -> Vec<V>;
    fn replace_mut(&mut self, searcher: F, value: V);
}

impl<F, V> ReplaceVec<F, V> for Vec<V>
where
    F: FnMut(&V) -> bool,
    V: PartialEq + Clone,
{
    fn replace(&self, searcher: F, value: V) -> Vec<V> {
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

    fn replace_mut(&mut self, searcher: F, value: V) {
        let item_to_replace_index = self.iter().position(searcher);
        if let Some(index) = item_to_replace_index {
            self[index] = value;
        }
    }
}
