use crate::prelude::OwnedBinaryTreeNode;

use super::dfs_inorder_next;

pub struct OwnedDFSInorderIterator<Node> 
    where Node: OwnedBinaryTreeNode {
    
    right_stack: Vec<Option<Node>>,
    item_stack: Vec<Node::OwnedValue>,
}

impl<Node> OwnedDFSInorderIterator<Node> 
    where Node: OwnedBinaryTreeNode {

    pub (crate) fn new(root: Node) -> OwnedDFSInorderIterator<Node> {
        let mut right_stack = Vec::new();
        right_stack.push(Some(root));

        OwnedDFSInorderIterator {
            right_stack,
            item_stack: Vec::new(),
        }
    }
}

impl<Node> Iterator for OwnedDFSInorderIterator<Node>
    where Node: OwnedBinaryTreeNode {
    
    type Item = Node::OwnedValue;
    
    dfs_inorder_next!(get_value_and_left_right);
}