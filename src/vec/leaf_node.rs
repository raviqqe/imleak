use super::node_ref::NodeRef;
use std::mem::MaybeUninit;

const MAX_LEN: usize = 32;

#[derive(Clone)]
pub struct LeafNode<T: Copy> {
    values: [MaybeUninit<T>; MAX_LEN],
    len: usize,
}

impl<T: Copy> LeafNode<T> {
    pub fn new(values: &[T]) -> NodeRef<T> {
        let mut leaf_node = Self {
            values: [MaybeUninit::uninit(); 32],
            len: 0,
        };

        for (index, value) in values.iter().enumerate() {
            leaf_node.values[index] = MaybeUninit::new(*value);
            leaf_node.len += 1;
        }

        leaf_node.into()
    }

    pub fn push_back(&self, value: T) -> Option<NodeRef<T>> {
        if self.len == MAX_LEN {
            None
        } else {
            let mut leaf_node = self.clone();

            leaf_node.values[leaf_node.len] = MaybeUninit::new(value);
            leaf_node.len += 1;

            Some(leaf_node.into())
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
