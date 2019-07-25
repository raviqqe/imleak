use super::node_ref::NodeRef;

#[derive(Clone, Copy)]
pub struct Slot<T: Copy> {
    node_ref: NodeRef<T>,
    accumulated_len: usize,
}

impl<T: Copy> Slot<T> {
    pub fn new(node_ref: NodeRef<T>, accumulated_len: usize) -> Self {
        Self {
            node_ref,
            accumulated_len: accumulated_len + node_ref.len(),
        }
    }

    pub fn node_ref(&self) -> NodeRef<T> {
        self.node_ref
    }

    pub fn accumulated_len(&self) -> usize {
        self.accumulated_len
    }
}
