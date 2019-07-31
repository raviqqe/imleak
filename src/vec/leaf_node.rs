use super::constants::MAX_SIZE;
use super::node_ref::NodeRef;
use std::fmt::{self, Debug, Formatter};
use std::mem::MaybeUninit;
use std::ops::Index;

#[derive(Clone)]
pub struct LeafNode<T: Copy> {
    values: [MaybeUninit<T>; MAX_SIZE],
    len: usize,
}

impl<T: Copy> LeafNode<T> {
    pub fn new(values: &[T]) -> NodeRef<T> {
        assert!(values.len() <= MAX_SIZE);

        let mut leaf_node = Self {
            values: [MaybeUninit::uninit(); 32],
            len: 0,
        };

        for value in values {
            leaf_node.append_value(*value);
        }

        leaf_node.into()
    }

    pub fn push_back(&self, value: T) -> Option<NodeRef<T>> {
        if self.len == MAX_SIZE {
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

    fn append_value(&mut self, value: T) {
        self.values[self.len] = MaybeUninit::new(value);
        self.len += 1;
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

impl<T: Copy> Index<usize> for &LeafNode<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            unreachable!()
        }

        unsafe { self.values[index].get_ref() }
    }
}

impl<T: Copy + PartialEq> PartialEq for LeafNode<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            return false;
        }

        for index in 0..self.len {
            if self.get(index) != other.get(index) {
                return false;
            }
        }

        true
    }
}

impl<T: Copy> Debug for LeafNode<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "LeafNode {{ len: {} }}", self.len)
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
