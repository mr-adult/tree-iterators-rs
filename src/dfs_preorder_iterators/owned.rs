use crate::prelude::OwnedTreeNode;

use super::dfs_preorder_next;

pub struct OwnedDFSPreorderIterator<Node>
    where Node: OwnedTreeNode {

    root: Option<Node>,
    traversal_stack: Vec<Node::OwnedChildren>,
}

impl<Node> OwnedDFSPreorderIterator<Node> 
    where Node: OwnedTreeNode {
        
    pub (crate) fn new(root: Node) -> OwnedDFSPreorderIterator<Node> {
        OwnedDFSPreorderIterator { 
            root: Some(root),
            traversal_stack: Vec::new()
        }
    }
}

impl<Node> Iterator for OwnedDFSPreorderIterator<Node> 
    where Node: OwnedTreeNode {
    
    type Item = Node::OwnedValue;
    dfs_preorder_next!(get_value_and_children);
}