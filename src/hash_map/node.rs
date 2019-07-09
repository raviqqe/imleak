use std::borrow::Borrow;
use std::hash::Hash;

pub trait Node<K: Hash + PartialEq, V>
where
    Self: Sized,
{
    fn insert(&self, k: K, v: V) -> (Self, bool);
    fn remove<Q: ?Sized + Eq + Hash>(&self, k: &Q) -> Option<Self>
    where
        K: Borrow<Q>;
    fn get<Q: ?Sized + Eq + Hash>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>;
    fn is_singleton(&self) -> bool;
}
