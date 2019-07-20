mod internal_node;
mod leaf_node;
mod node_ref;

use leaf_node::LeafNode;
use node_ref::NodeRef;
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct Vector<T> {
    root: NodeRef,
    size: usize,
    phantom: PhantomData<T>,
}

impl<T: Copy> Vector<T> {
    pub fn new() -> Self {
        Self {
            root: NodeRef::leaf::<T>(LeafNode::new()),
            size: 0,
            phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Vector;

    #[test]
    fn new() {
        Vector::<usize>::new();
    }
}
