use super::node::Node;
use std::borrow::Borrow;
use std::hash::Hash;

// TODO: Fix Eq and PartialEq impl.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Bucket<K, V> {
    vector: Vec<(K, V)>,
}

impl<K, V> Bucket<K, V> {
    pub fn new(k: K, v: V) -> Self {
        Self {
            vector: vec![(k, v)],
        }
    }
}

impl<K, V> Bucket<K, V> {
    pub fn size(&self) -> usize {
        self.vector.len()
    }
}

impl<K: PartialEq, V> Bucket<K, V> {
    fn find_index<Q: ?Sized + PartialEq>(&self, k: &Q) -> Option<usize>
    where
        K: Borrow<Q>,
    {
        for (i, (kk, _)) in self.vector.iter().enumerate() {
            if kk.borrow() == k {
                return Some(i);
            }
        }

        None
    }
}

impl<K: Clone + Hash + PartialEq, V: Clone> Node<K, V> for Bucket<K, V> {
    fn insert(&self, k: K, v: V) -> (Self, bool) {
        let mut kvs = self.vector.clone();

        match self.find_index(&k) {
            Some(i) => {
                kvs[i] = (k, v);
                (Self { vector: kvs }, false)
            }
            None => {
                kvs.push((k, v));
                (Self { vector: kvs }, true)
            }
        }
    }

    fn remove<Q: ?Sized + PartialEq>(&self, k: &Q) -> Option<Self>
    where
        K: Borrow<Q>,
    {
        self.find_index(k).map(|i| {
            let mut v = self.vector.clone();
            v.remove(i);
            Self { vector: v }
        })
    }

    fn get<Q: ?Sized + PartialEq>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
    {
        self.find_index(k).map(|i| &self.vector[i].1)
    }

    fn size(&self) -> usize {
        self.vector.len()
    }
}

#[derive(Clone, Debug)]
pub struct BucketIterator<'a, K, V> {
    bucket: &'a Bucket<K, V>,
    index: usize,
}

impl<'a, K, V> BucketIterator<'a, K, V> {
    pub fn new(bucket: &'a Bucket<K, V>) -> Self {
        Self { bucket, index: 0 }
    }
}

impl<'a, K, V> Iterator for BucketIterator<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.bucket.vector.get(self.index);
        self.index += 1;
        item.map(|(k, v)| (k, v))
    }
}

impl<'a, K, V> IntoIterator for &'a Bucket<K, V> {
    type IntoIter = BucketIterator<'a, K, V>;
    type Item = (&'a K, &'a V);

    fn into_iter(self) -> Self::IntoIter {
        BucketIterator::new(self)
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

        assert_eq!(b.size(), 1);

        let (bb, new) = b.insert(0, 0);

        assert!(new);
        assert_eq!(b.size(), 1);
        assert_eq!(bb.size(), 2);
    }

    #[test]
    fn delete() {
        let b = Bucket::new(42, 0);

        assert_eq!(b.remove(&42).unwrap().size(), 0);
        assert_eq!(b.insert(0, 0).0.remove(&42).unwrap(), Bucket::new(0, 0));
    }

    #[test]
    fn find() {
        let b = Bucket::new(42, 0);

        assert_eq!(b.get(&42), Some(&0));
        assert_eq!(b.get(&0), None);
    }
}
