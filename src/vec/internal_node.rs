use super::constants::MAX_SIZE;
use super::node_ref::NodeRef;
use super::slot::Slot;
use super::utilities::create_branch;
use std::mem::MaybeUninit;

#[derive(Clone)]
pub struct InternalNode<T: Copy> {
    slots: [MaybeUninit<Slot<T>>; MAX_SIZE],
    size: usize,
}

impl<T: Copy> InternalNode<T> {
    pub fn new(node_refs: &[NodeRef<T>]) -> NodeRef<T> {
        let mut internal_node = Self {
            slots: [MaybeUninit::uninit(); 32],
            size: 0,
        };

        for node_ref in node_refs {
            internal_node.append_slot(*node_ref);
        }

        internal_node.into()
    }

    pub fn push_back(&self, value: T) -> Option<NodeRef<T>> {
        match self.get(self.size - 1).node_ref().push_back(value) {
            None => {
                if self.size == MAX_SIZE {
                    None
                } else {
                    let mut internal_node = self.clone();
                    internal_node.update_slot(create_branch(value, self.level() - 1));

                    assert!(internal_node.balanced());

                    Some(internal_node.into())
                }
            }
            Some(node_ref) => {
                let mut internal_node = self.clone();
                internal_node.update_slot(node_ref);

                assert!(internal_node.balanced());

                Some(internal_node.into())
            }
        }
    }

    pub fn len(&self) -> usize {
        if self.size == 0 {
            0
        } else {
            self.get(self.size - 1).accumulated_len()
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn get(&self, index: usize) -> Slot<T> {
        unsafe { self.slots[index].read() }
    }

    pub fn level(&self) -> usize {
        self.get(self.size - 1).node_ref().level() + 1
    }

    pub fn balanced(&self) -> bool {
        let level = self.level() - 1;

        (0..self.size).all(|index| self.get(index).node_ref().level() == level)
    }

    fn append_slot(&mut self, node_ref: NodeRef<T>) {
        self.slots[self.size] = MaybeUninit::new(Slot::new(node_ref, self.len()));
        self.size += 1;
    }

    fn update_slot(&mut self, node_ref: NodeRef<T>) {
        assert!(self.size > 0);

        self.slots[self.size - 1] = MaybeUninit::new(Slot::new(
            node_ref,
            if self.size == 1 {
                0
            } else {
                self.get(self.size - 2).accumulated_len()
            },
        ));
    }
}

impl<'a, T: Copy> IntoIterator for &'a InternalNode<T> {
    type IntoIter = InternalNodeIterator<'a, T>;
    type Item = NodeRef<T>;

    fn into_iter(self) -> Self::IntoIter {
        InternalNodeIterator {
            internal_node: self,
            index: 0,
        }
    }
}

pub struct InternalNodeIterator<'a, T: Copy> {
    internal_node: &'a InternalNode<T>,
    index: usize,
}

impl<'a, T: Copy> Iterator for InternalNodeIterator<'a, T> {
    type Item = NodeRef<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.internal_node.size() {
            None
        } else {
            let node_ref = self.internal_node.get(self.index).node_ref();

            self.index += 1;

            Some(node_ref)
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::leaf_node::LeafNode;
    use super::super::node_ref::NodeRef;
    use super::InternalNode;

    #[test]
    fn new() {
        InternalNode::<usize>::new(&[]);
        InternalNode::new(&[LeafNode::new(&[42])]);
    }

    #[test]
    fn iterator() {
        let node_ref = InternalNode::new(&[LeafNode::new(&[42])]);
        let iter = node_ref.as_internal().unwrap().into_iter();

        assert_eq!(iter.collect::<Vec<NodeRef<usize>>>().len(), 1);

        let node_ref = InternalNode::new(&[LeafNode::new(&[42]), LeafNode::new(&[42])]);
        let iter = node_ref.as_internal().unwrap().into_iter();

        assert_eq!(iter.collect::<Vec<NodeRef<usize>>>().len(), 2);
    }
}
