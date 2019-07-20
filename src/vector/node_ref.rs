use super::internal_node::InternalNode;
use super::leaf_node::LeafNode;
use crate::tagged_ref::TaggedRef;

const ALIGNMENT: usize = 8;

#[derive(Clone, Copy, Debug)]
pub struct NodeRef {
    tagged_ref: TaggedRef,
}

impl NodeRef {
    pub fn internal(internal_node: InternalNode) -> Self {
        Self {
            tagged_ref: TaggedRef::new(internal_node, NodeTag::InternalNode.into()),
        }
    }

    pub fn leaf<T: Copy>(leaf_node: LeafNode<T>) -> Self {
        Self {
            tagged_ref: TaggedRef::new(leaf_node, NodeTag::LeafNode.into()),
        }
    }

    pub fn tag(&self) -> NodeTag {
        self.tagged_ref.tag().into()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum NodeTag {
    InternalNode,
    LeafNode,
}

impl From<NodeTag> for usize {
    fn from(tag: NodeTag) -> usize {
        match tag {
            NodeTag::InternalNode => 0,
            NodeTag::LeafNode => 1,
        }
    }
}

impl From<usize> for NodeTag {
    fn from(tag: usize) -> NodeTag {
        match tag {
            0 => NodeTag::InternalNode,
            1 => NodeTag::LeafNode,
            _ => unreachable!(),
        }
    }
}
