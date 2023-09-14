use crate::prelude::BorrowedBinaryTreeNode;

use super::dfs_inorder_next;

pub struct BorrowedDFSInorderIterator<'a, Node> 
    where Node: BorrowedBinaryTreeNode<'a> {
    
    right_stack: Vec<Option<&'a Node>>,
    item_stack: Vec<Node::BorrowedValue>,
}

impl<'a, Node> BorrowedDFSInorderIterator<'a, Node> 
    where Node: BorrowedBinaryTreeNode<'a> {

    pub (crate) fn new(root: &'a Node) -> BorrowedDFSInorderIterator<'a, Node> {
        let mut right_stack = Vec::new();
        right_stack.push(Some(root));

        BorrowedDFSInorderIterator {
            right_stack,
            item_stack: Vec::new(),
        }
    }
}

impl<'a, Node> Iterator for BorrowedDFSInorderIterator<'a, Node>
    where Node: BorrowedBinaryTreeNode<'a> {
    
    type Item = Node::BorrowedValue;
    
    dfs_inorder_next!(get_value_and_left_right_iter);
}