pub trait ReplaceVec<F, V>
where
    F: FnOnce() -> bool,
    V: PartialEq,
{
    fn replace(&self, searcher: F, value: V) -> &Self;
    fn replace_mut(&mut self, searcher: F, value: V);
}

impl<F, V> ReplaceVec<F, V> for Vec<V>
where
    F: FnOnce() -> bool,
    V: PartialEq,
{
    fn replace(&self, searcher: F, value: V) -> &Self {
        todo!()
    }

    fn replace_mut(&mut self, searcher: F, value: V) {
        todo!()
    }
}
