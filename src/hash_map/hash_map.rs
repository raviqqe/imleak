use std::borrow::Borrow;
use std::hash::Hash;
use std::sync::Arc;

use super::hamt::{HAMTIterator, HAMT};
use super::node::Node;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct HashMap<K, V> {
    size: usize,
    hamt: Arc<HAMT<K, V>>,
}

impl<K: Clone + Hash + PartialEq, V: Clone> HashMap<K, V> {
    pub fn new() -> Self {
        Self {
            size: 0,
            hamt: HAMT::new(0).into(),
        }
    }

    pub fn insert(&self, k: K, v: V) -> Self {
        let (h, b) = self.hamt.insert(k, v);

        Self {
            size: self.size + (b as usize),
            hamt: h.into(),
        }
    }

    pub fn remove<Q: ?Sized + Hash + PartialEq>(&self, k: &Q) -> Option<Self>
    where
        K: Borrow<Q>,
    {
        self.hamt.remove(k).map(|h| Self {
            size: self.size - 1,
            hamt: h.into(),
        })
    }

    pub fn get<Q: ?Sized + Hash + PartialEq>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
    {
        self.hamt.get(k)
    }

    pub fn first_rest(&self) -> Option<(&K, &V, Self)> {
        self.hamt.first_rest().map(|(k, v, h)| {
            (
                k,
                v,
                Self {
                    size: self.size - 1,
                    hamt: h.into(),
                },
            )
        })
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl<K: Clone + Hash + PartialEq, V: Clone> Default for HashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct HashMapIterator<'a, K: 'a, V: 'a>(HAMTIterator<'a, K, V>);

impl<'a, K, V> Iterator for HashMapIterator<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a, K, V> IntoIterator for &'a HashMap<K, V> {
    type IntoIter = HashMapIterator<'a, K, V>;
    type Item = (&'a K, &'a V);

    fn into_iter(self) -> Self::IntoIter {
        HashMapIterator(self.hamt.into_iter())
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

        assert_eq!(h.size(), 0);
        assert_eq!(h.insert(0, 0).size(), 1);
        assert_eq!(h.insert(0, 0).insert(0, 0).size(), 1);
        assert_eq!(h.insert(0, 0).insert(1, 0).size(), 2);
    }

    #[test]
    fn insert_many_in_order() {
        let mut h = HashMap::new();

        for i in 0..NUM_ITERATIONS {
            h = h.insert(i, i);
            assert_eq!(h.size(), i + 1);
        }
    }

    #[test]
    fn insert_many_at_random() {
        let mut h: HashMap<usize, usize> = HashMap::new();

        for i in 0..NUM_ITERATIONS {
            let k = random();
            h = h.insert(k, k);
            assert_eq!(h.size(), i + 1);
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
            let s = h.size();
            let found = h.get(&k).is_some();

            if random() {
                h = h.insert(k, k);

                assert_eq!(h.size(), if found { s } else { s + 1 });
                assert_eq!(h.get(&k), Some(&k));
            } else {
                h = h.remove(&k).unwrap_or(h);

                assert_eq!(h.size(), if found { s - 1 } else { s });
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
    fn first_rest() {
        let mut h: HashMap<i16, i16> = HashMap::new();

        for _ in 0..NUM_ITERATIONS {
            h = h.insert(random(), 0);
        }

        for _ in 0..h.size() {
            let (k, _, r) = h.first_rest().unwrap();

            assert_eq!(r.size(), h.size() - 1);
            assert_eq!(r.get(k), None);

            h = r;
        }

        assert_eq!(h, HashMap::new());
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
