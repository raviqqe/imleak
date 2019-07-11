use super::bucket::{Bucket, BucketIterator};
use super::entry::Entry;
use super::hashed_key::HashedKey;
use super::node::Node;
use std::borrow::Borrow;
use std::hash::Hash;

const MAX_LEVEL: u8 = 64 / 5;
const NUM_ENTRIES: usize = 32;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HAMT<K: Eq + Hash, V: PartialEq> {
    // TODO: Use bitmap.
    entries: [Entry<K, V>; NUM_ENTRIES],
}

impl<K: Clone + Hash + Eq, V: Clone + PartialEq> HAMT<K, V> {
    pub fn new() -> Self {
        Self {
            entries: Default::default(),
        }
    }

    pub fn insert(&self, k: HashedKey<K>, v: V) -> (Self, bool) {
        let i = k.entry_index();

        match &self.entries[i] {
            Entry::Empty => (self.set_entry(i, Entry::KeyValue(k.to_key(), v)), true),
            Entry::KeyValue(kk, vv) => {
                if kk == k.key() {
                    (self.set_entry(i, Entry::KeyValue(k.to_key(), v)), false)
                } else {
                    (
                        self.set_entry(
                            i,
                            if k.level() < MAX_LEVEL {
                                Entry::HAMT(
                                    Self::new()
                                        .insert(
                                            k.swap_key(kk.clone()).increment_level(),
                                            vv.clone(),
                                        )
                                        .0
                                        .insert(k.increment_level(), v)
                                        .0
                                        .into(),
                                )
                            } else {
                                Entry::Bucket(
                                    Bucket::new(kk.clone(), vv.clone())
                                        .insert(k.to_key(), v)
                                        .0
                                        .into(),
                                )
                            },
                        ),
                        true,
                    )
                }
            }
            Entry::HAMT(h) => {
                let (h, new) = h.insert(k.increment_level(), v);
                (self.set_entry(i, Entry::HAMT(h.into())), new)
            }
            Entry::Bucket(b) => {
                let (b, new) = b.insert(k.to_key(), v);
                (self.set_entry(i, Entry::Bucket(b.into())), new)
            }
        }
    }

    pub fn remove<Q: ?Sized + Eq + Hash>(&self, k: HashedKey<&Q>) -> Option<Self>
    where
        K: Borrow<Q>,
    {
        let i = k.entry_index();

        self.set_entry(
            i,
            match &self.entries[i] {
                Entry::Empty => return None,
                Entry::KeyValue(kk, _) => {
                    if &kk.borrow() == k.key() {
                        Entry::Empty
                    } else {
                        return None;
                    }
                }
                Entry::HAMT(h) => match h.remove(k.increment_level()) {
                    None => return None,
                    Some(h) => h.into(),
                },
                Entry::Bucket(b) => match b.remove(k.key()) {
                    None => return None,
                    Some(b) => b.into(),
                },
            },
        )
        .into()
    }

    pub fn get<Q: ?Sized + Eq + Hash>(&self, k: HashedKey<&Q>) -> Option<&V>
    where
        K: Borrow<Q>,
    {
        match &self.entries[k.entry_index()] {
            Entry::Empty => None,
            Entry::KeyValue(kk, vv) => {
                if &kk.borrow() == k.key() {
                    Some(vv)
                } else {
                    None
                }
            }
            Entry::HAMT(h) => h.get(k.increment_level()),
            Entry::Bucket(b) => b.get(k.key()),
        }
    }

    fn set_entry(&self, i: usize, e: Entry<K, V>) -> Self {
        let mut es = self.entries.clone();
        es[i] = e;

        Self { entries: es }
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.entries
            .iter()
            .map(|e| match e {
                Entry::Empty => 0,
                Entry::KeyValue(_, _) => 1,
                Entry::HAMT(h) => h.len(),
                Entry::Bucket(b) => b.len(),
            })
            .sum()
    }

    #[cfg(test)]
    fn contain_bucket(&self) -> bool {
        self.entries.iter().any(|e| match e {
            Entry::Bucket(_) => true,
            _ => false,
        })
    }

    #[cfg(test)]
    fn is_normal(&self) -> bool {
        self.entries.iter().all(|e| match e {
            Entry::Bucket(b) => b.len() != 1,
            Entry::HAMT(h) => h.is_normal() && h.len() != 1,
            _ => true,
        })
    }
}

impl<K: Eq + Hash, V: PartialEq> Node for HAMT<K, V> {
    fn is_singleton(&self) -> bool {
        let mut sum = 0;

        for e in &self.entries {
            match e {
                Entry::Empty => {}
                Entry::KeyValue(_, _) => sum += 1,
                Entry::HAMT(_) => return false,
                Entry::Bucket(_) => return false,
            }
        }

        sum == 1
    }
}

#[derive(Clone, Debug)]
pub struct HAMTIterator<'a, K: 'a + Eq + Hash, V: 'a + PartialEq> {
    hamts: Vec<(&'a HAMT<K, V>, usize)>,
    bucket_iterator: Option<BucketIterator<'a, K, V>>,
}

impl<'a, K: Eq + Hash, V: PartialEq> IntoIterator for &'a HAMT<K, V> {
    type IntoIter = HAMTIterator<'a, K, V>;
    type Item = (&'a K, &'a V);

    fn into_iter(self) -> Self::IntoIter {
        HAMTIterator {
            hamts: vec![(self, 0)],
            bucket_iterator: None,
        }
    }
}

impl<'a, K: Eq + Hash, V: PartialEq> Iterator for HAMTIterator<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.bucket_iterator {
            Some(b) => b.next().or_else(|| {
                self.bucket_iterator = None;
                self.next()
            }),
            None => self.hamts.pop().and_then(|(h, i)| {
                if i == NUM_ENTRIES {
                    self.next()
                } else {
                    self.hamts.push((h, i + 1));

                    match &h.entries[i] {
                        Entry::Empty => self.next(),
                        Entry::HAMT(h) => {
                            self.hamts.push((h, 0));
                            self.next()
                        }
                        Entry::KeyValue(k, v) => Some((k, v)),
                        Entry::Bucket(b) => {
                            self.bucket_iterator = b.into_iter().into();
                            self.next()
                        }
                    }
                }
            }),
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::hashed_key::HashedKey;
    use super::{HAMT, MAX_LEVEL};
    use rand::{random, seq::SliceRandom, thread_rng};
    use std::collections::HashMap;
    use test::Bencher;

    const NUM_ITERATIONS: usize = 1 << 12;

    #[test]
    fn new() {
        HAMT::new() as HAMT<usize, usize>;
    }

    #[test]
    fn insert() {
        let h = HAMT::new();

        assert_eq!(h.len(), 0);

        let (h, b) = h.insert(HashedKey::new(0), 0);

        assert!(b);
        assert_eq!(h.len(), 1);

        let (hh, b) = h.insert(HashedKey::new(0), 0);

        assert!(!b);
        assert_eq!(hh.len(), 1);

        let (h, b) = h.insert(HashedKey::new(1), 0);

        assert!(b);
        assert_eq!(h.len(), 2);
    }

    #[test]
    fn insert_many_in_order() {
        let mut h = HAMT::new();

        for i in 0..NUM_ITERATIONS {
            let (hh, b) = h.insert(HashedKey::new(i), i);
            h = hh;
            assert!(b);
            assert_eq!(h.len(), i + 1);
        }
    }

    #[test]
    fn insert_many_at_random() {
        let mut h: HAMT<usize, usize> = HAMT::new();

        for i in 0..NUM_ITERATIONS {
            let k = random();
            h = h.insert(HashedKey::new(k), k).0;
            assert_eq!(h.len(), i + 1);
        }
    }

    #[test]
    fn remove() {
        let h = HAMT::new();

        assert_eq!(
            h.insert(HashedKey::new(0), 0).0.remove(HashedKey::new(&0)),
            Some(h.clone())
        );
        assert_eq!(
            h.insert(HashedKey::new(0), 0).0.remove(HashedKey::new(&1)),
            None
        );
        assert_eq!(
            h.insert(HashedKey::new(0), 0)
                .0
                .insert(HashedKey::new(1), 0)
                .0
                .remove(HashedKey::new(&0)),
            Some(h.insert(HashedKey::new(1), 0).0)
        );
        assert_eq!(
            h.insert(HashedKey::new(0), 0)
                .0
                .insert(HashedKey::new(1), 0)
                .0
                .remove(HashedKey::new(&1)),
            Some(h.insert(HashedKey::new(0), 0).0)
        );
        assert_eq!(
            h.insert(HashedKey::new(0), 0)
                .0
                .insert(HashedKey::new(1), 0)
                .0
                .remove(HashedKey::new(&2)),
            None
        );
    }

    #[test]
    fn insert_delete_many() {
        let mut h: HAMT<i16, i16> = HAMT::new();

        for _ in 0..NUM_ITERATIONS {
            let k = random();
            let s = h.len();
            let found = h.get(HashedKey::new(&k)).is_some();

            if random() {
                h = h.insert(HashedKey::new(k), k).0;

                assert_eq!(h.len(), if found { s } else { s + 1 });
                assert_eq!(h.get(HashedKey::new(&k)), Some(&k));
            } else {
                h = h.remove(HashedKey::new(&k)).unwrap_or(h);

                assert_eq!(h.len(), if found { s - 1 } else { s });
                assert_eq!(h.get(HashedKey::new(&k)), None);
            }

            assert!(h.is_normal());
        }
    }

    #[test]
    fn get() {
        let h = HAMT::new();

        assert_eq!(
            h.insert(HashedKey::new(0), 0).0.get(HashedKey::new(&0)),
            Some(&0)
        );
        assert_eq!(
            h.insert(HashedKey::new(0), 0).0.get(HashedKey::new(&1)),
            None
        );
        assert_eq!(
            h.insert(HashedKey::new(1), 0).0.get(HashedKey::new(&0)),
            None
        );
        assert_eq!(
            h.insert(HashedKey::new(1), 0).0.get(HashedKey::new(&1)),
            Some(&0)
        );
        assert_eq!(
            h.insert(HashedKey::new(0), 0)
                .0
                .insert(HashedKey::new(1), 0)
                .0
                .get(HashedKey::new(&0)),
            Some(&0)
        );
        assert_eq!(
            h.insert(HashedKey::new(0), 0)
                .0
                .insert(HashedKey::new(1), 0)
                .0
                .get(HashedKey::new(&1)),
            Some(&0)
        );
        assert_eq!(
            h.insert(HashedKey::new(0), 0)
                .0
                .insert(HashedKey::new(1), 0)
                .0
                .get(HashedKey::new(&2)),
            None
        );
    }

    #[test]
    fn equality() {
        for _ in 0..8 {
            let mut hs: [HAMT<i16, i16>; 2] = [HAMT::new(), HAMT::new()];
            let mut is: Vec<i16> = (0..NUM_ITERATIONS).map(|_| random()).collect();
            let mut ds: Vec<i16> = (0..NUM_ITERATIONS).map(|_| random()).collect();

            for h in hs.iter_mut() {
                is.shuffle(&mut thread_rng());
                ds.shuffle(&mut thread_rng());

                for i in &is {
                    *h = h.insert(HashedKey::new(*i), *i).0;
                }

                for d in &ds {
                    *h = h.remove(HashedKey::new(&d)).unwrap_or(h.clone());
                }
            }

            assert_eq!(hs[0], hs[1]);
        }
    }

    #[test]
    fn collision() {
        let mut h = HAMT::new();

        for k in 0..33 {
            let mut hk = HashedKey::new(k);

            for _ in 0..MAX_LEVEL {
                hk = hk.increment_level()
            }

            h = h.insert(hk, k).0;
        }

        assert!(h.contain_bucket());
    }

    #[test]
    fn iterator() {
        let mut ss: Vec<usize> = (0..42).collect();

        for _ in 0..100 {
            ss.push(random::<usize>() % 1024);
        }

        for l in vec![0, MAX_LEVEL] {
            for s in &ss {
                let mut h: HAMT<i16, i16> = HAMT::new();
                let mut m: HashMap<i16, i16> = HashMap::new();

                for _ in 0..*s {
                    let k = random();
                    let v = random();

                    let mut hk = HashedKey::new(k);

                    for _ in 0..l {
                        hk = hk.increment_level()
                    }

                    let (hh, _) = h.insert(hk, v);
                    h = hh;

                    m.insert(k, v);
                }

                let mut s = 0;

                for (k, v) in &h {
                    s += 1;

                    assert_eq!(m[k], *v);
                }

                assert_eq!(s, h.len());
            }
        }
    }

    #[test]
    fn iterate_with_buckets() {
        let ks = (0..1000).collect::<Vec<_>>();

        let mut h = HAMT::new();

        for k in &ks {
            let mut hk = HashedKey::new(k);

            for _ in 0..MAX_LEVEL {
                hk = hk.increment_level()
            }
            h = h.insert(hk, k).0;
        }

        assert_eq!(ks.len(), h.into_iter().collect::<Vec<_>>().len())
    }

    fn keys() -> Vec<i16> {
        (0..1000).collect()
    }

    #[bench]
    fn bench_insert_1000(b: &mut Bencher) {
        let ks = keys();

        b.iter(|| {
            let mut h = HAMT::new();

            for k in &ks {
                h = h.insert(HashedKey::new(k), k).0;
            }
        });
    }

    #[bench]
    fn bench_get_1000(b: &mut Bencher) {
        let ks = keys();
        let mut h = HAMT::new();

        for k in &ks {
            h = h.insert(HashedKey::new(k), k).0;
        }

        b.iter(|| {
            for k in &ks {
                h.get(HashedKey::new(&k));
            }
        });
    }

    #[bench]
    fn bench_hash_map_insert_1000(b: &mut Bencher) {
        let ks = keys();

        b.iter(|| {
            let mut h = HashMap::new();

            for k in &ks {
                h.insert(k, k);
            }
        });
    }

    #[bench]
    fn bench_hash_map_get_1000(b: &mut Bencher) {
        let ks = keys();
        let mut h = HashMap::new();

        for k in &ks {
            h.insert(k, k);
        }

        b.iter(|| {
            for k in &ks {
                h.get(&k);
            }
        });
    }
}
