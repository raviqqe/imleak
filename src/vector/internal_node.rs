use super::node_ref::NodeRef;
use super::utilities::create_branch;
use std::mem::MaybeUninit;

const MAX_SIZE: usize = 32;

#[derive(Clone)]
pub struct InternalNode {
    node_refs: [MaybeUninit<NodeRef>; MAX_SIZE],
    size: usize,
}

impl InternalNode {
    pub fn new(node_refs: &[NodeRef]) -> Self {
        let mut internal_node = Self {
            node_refs: [MaybeUninit::uninit(); 32],
            size: 0,
        };

        for (index, node_ref) in node_refs.iter().enumerate() {
            internal_node.node_refs[index] = MaybeUninit::new(*node_ref);
            internal_node.size += 1;
        }

        internal_node
    }

    pub fn push_back<T: Copy>(&self, value: T) -> Option<Self> {
        match unsafe { self.node_refs[self.size - 1].read() }.push_back(value) {
            None => {
                if self.size == MAX_SIZE {
                    None
                } else {
                    let mut internal_node = self.clone();
                    internal_node.node_refs[self.size - 1] =
                        MaybeUninit::new(create_branch(value, self.level::<T>() - 1));

                    assert!(internal_node.balanced::<T>());

                    Some(internal_node)
                }
            }
            Some(node_ref) => {
                let mut internal_node = self.clone();
                internal_node.node_refs[self.size - 1] = MaybeUninit::new(node_ref);

                assert!(internal_node.balanced::<T>());

                Some(internal_node)
            }
        }
    }

    pub fn level<T: Copy>(&self) -> usize {
        unsafe { self.node_refs[self.size - 1].read() }.level::<T>() + 1
    }

    pub fn balanced<T: Copy>(&self) -> bool {
        let level = self.level::<T>() - 1;

        self.node_refs[..self.size]
            .iter()
            .all(|node_ref| unsafe { node_ref.read() }.level::<T>() == level)
    }
}
