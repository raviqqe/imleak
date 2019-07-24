mod internal_node;
mod leaf_node;
mod node_ref;
mod utilities;

use internal_node::InternalNode;
use leaf_node::LeafNode;
use node_ref::NodeRef;
use std::marker::PhantomData;
use utilities::create_branch;

#[derive(Clone, Debug)]
pub struct Vector<T> {
    root: NodeRef,
    len: usize,
    phantom: PhantomData<T>,
}

impl<T: Copy> Vector<T> {
    pub fn new() -> Self {
        Self {
            root: NodeRef::leaf::<T>(LeafNode::new(&[])),
            len: 0,
            phantom: PhantomData,
        }
    }

    pub fn push_back(&self, value: T) -> Self {
        Self {
            root: self.root.push_back(value).unwrap_or_else(|| {
                InternalNode::new(&[self.root, create_branch(value, self.root.level::<T>())]).into()
            }),
            len: self.len + 1,
            phantom: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

#[cfg(test)]
mod test {
    use super::Vector;

    #[test]
    fn new() {
        Vector::<usize>::new();
    }

    #[test]
    fn push_back() {
        let mut vec = Vector::<usize>::new();

        for index in 0..1000 {
            vec = vec.push_back(index);

            assert_eq!(vec.len(), index + 1);
        }
    }
}
