use super::internal_node::InternalNode;
use super::leaf_node::LeafNode;
use crate::tagged_ref::TaggedRef;

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

    pub fn as_ref<T: Copy>(&self) -> ConcreteNodeRef<T> {
        match self.tagged_ref.tag().into() {
            NodeTag::InternalNode => ConcreteNodeRef::InternalNode(self.tagged_ref.as_ptr()),
            NodeTag::LeafNode => ConcreteNodeRef::LeafNode(self.tagged_ref.as_ptr()),
        }
    }

    pub fn push_back<T: Copy>(&self, value: T) -> Option<NodeRef> {
        match self.as_ref() {
            ConcreteNodeRef::InternalNode(internal_node) => {
                internal_node.push_back(value).map(Self::internal)
            }
            ConcreteNodeRef::LeafNode(leaf_node) => leaf_node.push_back(value).map(Self::leaf),
        }
    }

    pub fn level<T: Copy>(&self) -> usize {
        match self.as_ref::<T>() {
            ConcreteNodeRef::InternalNode(internal_node) => internal_node.level::<T>(),
            ConcreteNodeRef::LeafNode(_) => 0,
        }
    }
}

impl From<InternalNode> for NodeRef {
    fn from(internal_node: InternalNode) -> Self {
        NodeRef::internal(internal_node)
    }
}

impl<T: Copy> From<LeafNode<T>> for NodeRef {
    fn from(leaf_node: LeafNode<T>) -> Self {
        NodeRef::leaf(leaf_node)
    }
}

#[derive(Clone)]
pub enum ConcreteNodeRef<'a, T: Copy> {
    InternalNode(&'a InternalNode),
    LeafNode(&'a LeafNode<T>),
}

#[derive(Clone, Copy, Debug)]
enum NodeTag {
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
