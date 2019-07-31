mod constants;
mod internal_node;
mod leaf_node;
mod node_ref;
mod slot;
mod utilities;

use internal_node::InternalNode;
use leaf_node::LeafNode;
use node_ref::{ConcreteNodeRef, NodeRef};
use std::ops::Index;
use utilities::create_branch;

#[derive(Clone, Debug)]
pub struct Vec<T: Copy> {
    root: NodeRef<T>,
    len: usize,
}

impl<T: Copy> Vec<T> {
    pub fn new() -> Self {
        Self {
            root: LeafNode::new(&[]),
            len: 0,
        }
    }

    pub fn push_back(&self, value: T) -> Self {
        Self {
            root: self.root.push_back(value).unwrap_or_else(|| {
                InternalNode::new(&[self.root, create_branch(value, self.root.level())]).into()
            }),
            len: self.len + 1,
        }
    }

    pub fn append(&self, other: &Vec<T>) -> Vec<T> {
        match self.root.as_ref() {
            ConcreteNodeRef::InternalNode(internal_node) => {
                let mut root = Some(internal_node);
                let mut rest = None;

                for index in 0.. {
                    match (
                        self.root.as_internal().unwrap().right_internal(index),
                        other.root.left_internal(0),
                    ) {
                        (None, _) => {
                            return Self {
                                root,
                                len: leaf_node.len(),
                            }
                        }
                        (_, None) => {
                            return Self {
                                root: leaf_node,
                                len: leaf_node.len(),
                            }
                        }
                        (Some(left_internal_node), Some(right_internal_node)) => {
                            let (root, new_rest) =
                                left_internal_node.append(rest, right_internal_node);
                        }
                    }
                }

                unreachable!()
            }
            ConcreteNodeRef::LeafNode(leaf_node) => unimplemented!(),
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T: Copy> Index<usize> for Vec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.root.index(index)
    }
}

impl<'a, T: Copy> IntoIterator for &'a Vec<T> {
    type IntoIter = VecIterator<'a, T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        VecIterator {
            vec: self,
            index: 0,
        }
    }
}

pub struct VecIterator<'a, T: Copy> {
    vec: &'a Vec<T>,
    index: usize,
}

impl<'a, T: Copy> Iterator for VecIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.vec.len() {
            None
        } else {
            let value = self.vec[self.index];
            self.index += 1;
            Some(value)
        }
    }
}

#[cfg(test)]
mod test {
    use super::Vec;

    const ITERATIONS: usize = 2000;

    #[test]
    fn new() {
        Vec::<usize>::new();
    }

    #[test]
    fn push_back() {
        let mut vec = Vec::<usize>::new();

        for index in 0..ITERATIONS {
            vec = vec.push_back(index);

            assert_eq!(vec.len(), index + 1);
        }
    }

    #[test]
    fn index() {
        let mut vec = Vec::<usize>::new();

        for value in 0..ITERATIONS {
            vec = vec.push_back(value);

            for index in 0..(value + 1) {
                assert_eq!(vec[index], index);
            }
        }
    }

    #[test]
    fn iterator() {
        let mut vec = Vec::<usize>::new();
        let mut std_vec = vec![];

        for value in 0..ITERATIONS {
            vec = vec.push_back(value);
            std_vec.push(value);
        }

        assert_eq!(vec.into_iter().collect::<std::vec::Vec<usize>>(), std_vec)
    }
}
