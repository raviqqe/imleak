use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HashedKey<K> {
    key: K,
    hash: u64,
    level: u8,
}

impl<K: Hash> HashedKey<K> {
    pub fn new(k: K) -> Self {
        let h = Self::hash(&k);

        Self {
            key: k,
            hash: h,
            level: 0,
        }
    }

    pub fn key(&self) -> &K {
        &self.key
    }

    pub fn entry_index(&self) -> usize {
        (self.hash & 0b11111) as usize
    }

    pub fn level(&self) -> u8 {
        self.level
    }

    pub fn swap_key(&self, k: K) -> Self {
        let h = Self::hash(&k);

        Self {
            key: k,
            hash: h >> (self.level * 5),
            level: self.level,
        }
    }

    pub fn increment_level(self) -> Self {
        Self {
            key: self.key,
            hash: self.hash >> 5,
            level: self.level + 1,
        }
    }

    pub fn to_key(self) -> K {
        self.key
    }

    fn hash(k: &K) -> u64 {
        let mut h = DefaultHasher::new();
        k.hash(&mut h);
        h.finish()
    }
}
