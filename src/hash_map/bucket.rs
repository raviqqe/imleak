use super::node::Node;
use std::borrow::Borrow;
use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Bucket<K: Eq + Hash, V: PartialEq> {
    hash_map: HashMap<K, V>,
}

impl<K: Eq + Hash, V: PartialEq> Bucket<K, V> {
    pub fn new(k: K, v: V) -> Self {
        Self {
            hash_map: vec![(k, v)].into_iter().collect(),
        }
    }
}

impl<K: Clone + Eq + Hash, V: Clone + PartialEq> Bucket<K, V> {
    pub fn insert(&self, k: K, v: V) -> (Self, bool) {
        let mut h = self.hash_map.clone();

        match h.insert(k, v) {
            Some(_) => (Self { hash_map: h }, false),
            None => (Self { hash_map: h }, true),
        }
    }

    pub fn remove<Q: ?Sized + Eq + Hash>(&self, k: &Q) -> Option<Self>
    where
        K: Borrow<Q>,
    {
        let mut h = self.hash_map.clone();
        h.remove(k).map(|_| Self { hash_map: h })
    }

    pub fn get<Q: ?Sized + Eq + Hash>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
    {
        self.hash_map.get(k)
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.hash_map.len()
    }
}

impl<K: Eq + Hash, V: PartialEq> Node for Bucket<K, V> {
    fn is_singleton(&self) -> bool {
        self.hash_map.len() == 1
    }
}

#[derive(Clone, Debug)]
pub struct BucketIterator<'a, K, V> {
    iterator: Iter<'a, K, V>,
}

impl<'a, K, V> Iterator for BucketIterator<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}

impl<'a, K: Eq + Hash, V: PartialEq> IntoIterator for &'a Bucket<K, V> {
    type IntoIter = BucketIterator<'a, K, V>;
    type Item = (&'a K, &'a V);

    fn into_iter(self) -> Self::IntoIter {
        BucketIterator {
            iterator: self.hash_map.iter(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        Bucket::new(42, 0);
    }

    #[test]
    fn insert() {
        let b = Bucket::new(42, 0);

        assert_eq!(b.len(), 1);

        let (bb, new) = b.insert(0, 0);

        assert!(new);
        assert_eq!(b.len(), 1);
        assert_eq!(bb.len(), 2);
    }

    #[test]
    fn remove() {
        let b = Bucket::new(42, 0);

        assert_eq!(b.remove(&42).unwrap().len(), 0);
        assert_eq!(b.insert(0, 0).0.remove(&42), Some(Bucket::new(0, 0)));
    }

    #[test]
    fn get() {
        let b = Bucket::new(42, 0);

        assert_eq!(b.get(&42), Some(&0));
        assert_eq!(b.get(&0), None);
    }

    #[test]
    fn eq() {
        assert!(Bucket::new(0, 0).insert(1, 0) == Bucket::new(0, 0).insert(1, 0));
        assert!(Bucket::new(0, 0).insert(1, 0) == Bucket::new(1, 0).insert(0, 0));
    }
}
