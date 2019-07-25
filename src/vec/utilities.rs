use super::internal_node::InternalNode;
use super::leaf_node::LeafNode;
use super::node_ref::NodeRef;

pub fn create_branch<T: Copy>(value: T, level: usize) -> NodeRef<T> {
    if level == 0 {
        LeafNode::new(&[value]).into()
    } else {
        InternalNode::new(&[create_branch(value, level - 1)]).into()
    }
}
