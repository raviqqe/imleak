use super::node_ref::NodeRef;
use super::slot::Slot;
use super::utilities::create_branch;
use std::mem::MaybeUninit;

const MAX_SIZE: usize = 32;

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
        match unsafe { self.slots[self.size - 1].read() }
            .node_ref()
            .push_back(value)
        {
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
            unsafe { self.slots[self.size - 1].read() }.accumulated_len()
        }
    }

    pub fn level(&self) -> usize {
        unsafe { self.slots[self.size - 1].read() }
            .node_ref()
            .level()
            + 1
    }

    pub fn balanced(&self) -> bool {
        let level = self.level() - 1;

        self.slots[..self.size]
            .iter()
            .all(|node_ref| unsafe { node_ref.read() }.node_ref().level() == level)
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
                unsafe { self.slots[self.size - 2].read() }.accumulated_len()
            },
        ));
    }
}
