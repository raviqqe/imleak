mod constants;
mod internal_node;
mod leaf_node;
mod node_ref;
mod slot;
mod utilities;

use internal_node::InternalNode;
use leaf_node::LeafNode;
use node_ref::NodeRef;
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

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

#[cfg(test)]
mod test {
    use super::Vec;

    #[test]
    fn new() {
        Vec::<usize>::new();
    }

    #[test]
    fn push_back() {
        let mut vec = Vec::<usize>::new();

        for index in 0..1000 {
            vec = vec.push_back(index);

            assert_eq!(vec.len(), index + 1);
        }
    }
}
