use std::hash::Hash;

pub trait Node<K: Hash + PartialEq, V>
where
    Self: Sized,
{
    // TODO: Move out the removed value as Option<Value>.
    fn insert(&self, K, V) -> (Self, bool);
    fn remove(&self, &K) -> Option<Self>;
    fn get(&self, &K) -> Option<&V>;
    fn first_rest(&self) -> Option<(&K, &V, Self)>;
    fn is_singleton(&self) -> bool; // for normalization
    fn size(&self) -> usize; // for debugging
}
