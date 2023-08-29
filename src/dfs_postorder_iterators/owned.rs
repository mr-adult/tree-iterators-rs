use crate::prelude::OwnedTreeNode;

use super::dfs_postorder_next;

pub struct OwnedDFSPostorderIterator<Node> 
    where Node: OwnedTreeNode {

    root: Option<Node>,
    item_stack: Vec<Node::OwnedValue>,
    traversal_stack: Vec<Node::OwnedChildren>
}

impl<Node> OwnedDFSPostorderIterator<Node> 
    where Node: OwnedTreeNode {

    pub (crate) fn new(root: Node) -> OwnedDFSPostorderIterator<Node> {
        OwnedDFSPostorderIterator { 
            root: Some(root),
            item_stack: Vec::new(), 
            traversal_stack: Vec::new() 
        }
    }
}

impl<Node> Iterator for OwnedDFSPostorderIterator<Node> 
    where Node: OwnedTreeNode {

    type Item = Node::OwnedValue;
    dfs_postorder_next!(get_value_and_children);
}