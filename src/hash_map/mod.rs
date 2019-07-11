mod bucket;
mod entry;
mod hamt;
mod hashed_key;
mod node;

use hamt::{HAMTIterator, HAMT};
use hashed_key::HashedKey;
use std::borrow::Borrow;
use std::hash::Hash;
use std::sync::Arc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HashMap<K: Eq + Hash, V: PartialEq> {
    len: usize,
    hamt: Arc<HAMT<K, V>>,
}

impl<K: Clone + Eq + Hash, V: Clone + PartialEq> HashMap<K, V> {
    pub fn new() -> Self {
        Self {
            len: 0,
            hamt: HAMT::new().into(),
        }
    }

    pub fn insert(&self, k: K, v: V) -> Self {
        let (h, b) = self.hamt.insert(HashedKey::new(k), v);

        Self {
            len: self.len + (b as usize),
            hamt: h.into(),
        }
    }

    pub fn remove<Q: ?Sized + Eq + Hash>(&self, k: &Q) -> Option<Self>
    where
        K: Borrow<Q>,
    {
        self.hamt.remove(HashedKey::new(k)).map(|h| Self {
            len: self.len - 1,
            hamt: h.into(),
        })
    }

    pub fn get<Q: ?Sized + Eq + Hash>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
    {
        self.hamt.get(HashedKey::new(k))
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<K: Clone + Eq + Hash, V: Clone + PartialEq> Default for HashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct HashMapIterator<'a, K: 'a + Eq + Hash, V: 'a + PartialEq> {
    hamt_iterator: HAMTIterator<'a, K, V>,
}

impl<'a, K: Eq + Hash, V: PartialEq> Iterator for HashMapIterator<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.hamt_iterator.next()
    }
}

impl<'a, K: Eq + Hash, V: PartialEq> IntoIterator for &'a HashMap<K, V> {
    type IntoIter = HashMapIterator<'a, K, V>;
    type Item = (&'a K, &'a V);

    fn into_iter(self) -> Self::IntoIter {
        HashMapIterator {
            hamt_iterator: self.hamt.into_iter(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::HashMap;
    use rand::{random, seq::SliceRandom, thread_rng};
    use std::thread::spawn;
    use test::Bencher;

    const NUM_ITERATIONS: usize = 1 << 12;

    #[test]
    fn new() {
        HashMap::<usize, usize>::new();
    }

    #[test]
    fn insert() {
        let h = HashMap::new();

        assert_eq!(h.len(), 0);
        assert_eq!(h.insert(0, 0).len(), 1);
        assert_eq!(h.insert(0, 0).insert(0, 0).len(), 1);
        assert_eq!(h.insert(0, 0).insert(1, 0).len(), 2);
    }

    #[test]
    fn insert_many_in_order() {
        let mut h = HashMap::new();

        for i in 0..NUM_ITERATIONS {
            h = h.insert(i, i);
            assert_eq!(h.len(), i + 1);
        }
    }

    #[test]
    fn insert_many_at_random() {
        let mut h: HashMap<usize, usize> = HashMap::new();

        for i in 0..NUM_ITERATIONS {
            let k = random();
            h = h.insert(k, k);
            assert_eq!(h.len(), i + 1);
        }
    }

    #[test]
    fn remove() {
        let h = HashMap::new();

        assert_eq!(h.insert(0, 0).remove(&0), Some(h.clone()));
        assert_eq!(h.insert(0, 0).remove(&1), None);
        assert_eq!(h.insert(0, 0).insert(1, 0).remove(&0), Some(h.insert(1, 0)));
        assert_eq!(h.insert(0, 0).insert(1, 0).remove(&1), Some(h.insert(0, 0)));
        assert_eq!(h.insert(0, 0).insert(1, 0).remove(&2), None);
    }

    #[test]
    fn insert_remove_many() {
        let mut h: HashMap<i16, i16> = HashMap::new();

        for _ in 0..NUM_ITERATIONS {
            let k = random();
            let s = h.len();
            let found = h.get(&k).is_some();

            if random() {
                h = h.insert(k, k);

                assert_eq!(h.len(), if found { s } else { s + 1 });
                assert_eq!(h.get(&k), Some(&k));
            } else {
                h = h.remove(&k).unwrap_or(h);

                assert_eq!(h.len(), if found { s - 1 } else { s });
                assert_eq!(h.get(&k), None);
            }
        }
    }

    #[test]
    fn get() {
        let h = HashMap::new();

        assert_eq!(h.insert(0, 0).get(&0), Some(&0));
        assert_eq!(h.insert(0, 0).get(&1), None);
        assert_eq!(h.insert(1, 0).get(&0), None);
        assert_eq!(h.insert(1, 0).get(&1), Some(&0));
        assert_eq!(h.insert(0, 0).insert(1, 0).get(&0), Some(&0));
        assert_eq!(h.insert(0, 0).insert(1, 0).get(&1), Some(&0));
        assert_eq!(h.insert(0, 0).insert(1, 0).get(&2), None);
    }

    #[test]
    fn equality() {
        for _ in 0..8 {
            let mut hs: [HashMap<i16, i16>; 2] = [HashMap::new(), HashMap::new()];
            let mut is: Vec<i16> = (0..NUM_ITERATIONS).map(|_| random()).collect();
            let mut ds: Vec<i16> = (0..NUM_ITERATIONS).map(|_| random()).collect();

            for h in hs.iter_mut() {
                is.shuffle(&mut thread_rng());
                ds.shuffle(&mut thread_rng());

                for i in &is {
                    *h = h.insert(*i, *i);
                }

                for d in &ds {
                    *h = h.remove(d).unwrap_or(h.clone());
                }
            }

            assert_eq!(hs[0], hs[1]);
        }
    }

    #[test]
    fn send_and_sync() {
        let m: HashMap<usize, usize> = HashMap::new();
        spawn(move || m);
        let m: HashMap<String, String> = HashMap::new();
        spawn(move || m);
    }

    fn keys() -> Vec<i16> {
        (0..1000).collect()
    }

    #[bench]
    fn bench_insert_1000(b: &mut Bencher) {
        let ks = keys();

        b.iter(|| {
            let mut h = HashMap::new();

            for k in &ks {
                h = h.insert(k, k);
            }
        });
    }

    #[bench]
    fn bench_get_1000(b: &mut Bencher) {
        let ks = keys();
        let mut h = HashMap::new();

        for k in &ks {
            h = h.insert(k, k);
        }

        b.iter(|| {
            for k in &ks {
                h.get(&k);
            }
        });
    }
}
