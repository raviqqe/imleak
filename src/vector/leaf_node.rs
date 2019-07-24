use std::mem::MaybeUninit;

const MAX_SIZE: usize = 32;

#[derive(Clone)]
pub struct LeafNode<T: Copy> {
    values: [MaybeUninit<T>; MAX_SIZE],
    size: usize,
}

impl<T: Copy> LeafNode<T> {
    pub fn new(values: &[T]) -> Self {
        let mut leaf_node = Self {
            values: [MaybeUninit::uninit(); 32],
            size: 0,
        };

        for (index, value) in values.iter().enumerate() {
            leaf_node.values[index] = MaybeUninit::new(*value);
            leaf_node.size += 1;
        }

        leaf_node
    }

    pub fn push_back(&self, value: T) -> Option<Self> {
        if self.size == MAX_SIZE {
            None
        } else {
            let mut leaf_node = self.clone();

            leaf_node.values[leaf_node.size] = MaybeUninit::new(value);
            leaf_node.size += 1;

            Some(leaf_node)
        }
    }
}
