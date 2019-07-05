use std::borrow::Borrow;
use std::hash::Hash;

pub trait Node<K: Hash + PartialEq, V>
where
    Self: Sized,
{
    // TODO: Move out the removed value as Option<Value>.
    fn insert(&self, k: K, v: V) -> (Self, bool);
    fn remove<Q: ?Sized + Hash + PartialEq>(&self, k: &Q) -> Option<Self>
    where
        K: Borrow<Q>;
    fn get<Q: ?Sized + Hash + PartialEq>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>;
    fn first_rest(&self) -> Option<(&K, &V, Self)>;
    fn is_singleton(&self) -> bool; // for normalization
    fn size(&self) -> usize; // for debugging
}
