use super::bucket::Bucket;
use super::hamt::HAMT;
use super::node::Node;
use std::hash::Hash;
use std::sync::Arc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Entry<K: Eq + Hash, V: PartialEq> {
    Empty,
    KeyValue(K, V),
    HAMT(Arc<HAMT<K, V>>),
    Bucket(Arc<Bucket<K, V>>),
}

impl<K: Eq + Hash, V: PartialEq> Default for Entry<K, V> {
    fn default() -> Self {
        Entry::Empty
    }
}

impl<K: Clone + Eq + Hash, V: Clone + PartialEq> From<HAMT<K, V>> for Entry<K, V> {
    fn from(h: HAMT<K, V>) -> Self {
        convert_node_to_key_value(&h).unwrap_or_else(|| Entry::HAMT(h.into()))
    }
}

impl<K: Clone + Eq + Hash, V: Clone + PartialEq> From<Bucket<K, V>> for Entry<K, V> {
    fn from(b: Bucket<K, V>) -> Self {
        convert_node_to_key_value(&b).unwrap_or_else(|| Entry::Bucket(b.into()))
    }
}

fn convert_node_to_key_value<'a, K: 'a + Clone + Eq + Hash, V: 'a + Clone + PartialEq, N: Node>(
    n: &'a N,
) -> Option<Entry<K, V>>
where
    &'a N: IntoIterator<Item = (&'a K, &'a V)>,
{
    if n.is_singleton() {
        let (k, v) = n.into_iter().next().expect("non-empty node");
        Entry::KeyValue(k.clone(), v.clone()).into()
    } else {
        None
    }
}
