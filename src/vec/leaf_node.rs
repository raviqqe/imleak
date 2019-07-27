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

    pub fn get(&self, index: usize) -> T {
        unsafe { self.values[index].read() }
    }
}

impl<'a, T: Copy> IntoIterator for &'a LeafNode<T> {
    type IntoIter = LeafNodeIterator<'a, T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        LeafNodeIterator {
            leaf_node: self,
            index: 0,
        }
    }
}

pub struct LeafNodeIterator<'a, T: Copy> {
    leaf_node: &'a LeafNode<T>,
    index: usize,
}

impl<'a, T: Copy> Iterator for LeafNodeIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.leaf_node.len() {
            None
        } else {
            let value = self.leaf_node.get(self.index);

            self.index += 1;

            Some(value)
        }
    }
}

#[cfg(test)]
mod test {
    use super::LeafNode;

    #[test]
    fn new() {
        LeafNode::<usize>::new(&[]);
        LeafNode::new(&[42]);
    }

    #[test]
    fn iterator() {
        let node_ref = LeafNode::new(&[42]);
        let iter = node_ref.as_leaf().unwrap().into_iter();

        assert_eq!(iter.collect::<Vec<usize>>().len(), 1);

        let node_ref = LeafNode::new(&[42, 42]);
        let iter = node_ref.as_leaf().unwrap().into_iter();

        assert_eq!(iter.collect::<Vec<usize>>().len(), 2);
    }
}
