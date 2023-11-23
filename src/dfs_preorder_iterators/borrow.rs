use alloc::vec::Vec;
use streaming_iterator::StreamingIterator;
use crate::{
    prelude::{
        BorrowedTreeNode, 
        BinaryChildren, 
        BorrowedBinaryTreeNode, TreeIterator, AncestorsIterator, LeavesIterator
    }, 
    leaves_iterators::depth_first::borrow::{
        BorrowedLeavesIterator, 
        BorrowedBinaryLeavesIterator
    },
};

use super::{
    preorder_streaming_iterator_impl, 
    dfs_preorder_next, 
    advance_dfs
};

pub struct BorrowedDFSPreorderIterator<'a, Node>
    where Node: BorrowedTreeNode<'a> {

    root: Option<&'a Node>,
    traversal_stack: Vec<Node::BorrowedChildren>,
}

impl<'a, Node> BorrowedDFSPreorderIterator<'a, Node> 
    where Node: BorrowedTreeNode<'a> {
        
    pub (crate) fn new(root: &'a Node) -> BorrowedDFSPreorderIterator<'a, Node> {
        BorrowedDFSPreorderIterator { 
            root: Some(root),
            traversal_stack: Vec::new()
        }
    }
}

impl<'a, Node> TreeIterator for BorrowedDFSPreorderIterator<'a, Node> 
    where Node: BorrowedTreeNode<'a> {
    fn leaves(self) -> impl LeavesIterator<Item = Self::Item> {
        BorrowedLeavesIterator {
            root: self.root,
            traversal_stack_bottom: self.traversal_stack,
            traversal_stack_top: Vec::new(),
            item_stack: Vec::new(),
        }
    }
    
    fn attach_ancestors(self) -> impl AncestorsIterator<Item = [Node::BorrowedValue]> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS preorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                BorrowedDFSPreorderIteratorWithAncestors::new(root)
            }
        }
    }
}

impl<'a, Node> Iterator for BorrowedDFSPreorderIterator<'a, Node> 
    where Node: BorrowedTreeNode<'a> {
    
    type Item = Node::BorrowedValue;
    dfs_preorder_next!(get_value_and_children_iter);
}

pub struct BorrowedDFSPreorderIteratorWithAncestors<'a, Node>
    where Node: BorrowedTreeNode<'a> {

    root: Option<&'a Node>,
    traversal_stack: Vec<Node::BorrowedChildren>,
    item_stack: Vec<Node::BorrowedValue>,
}


impl<'a, Node> BorrowedDFSPreorderIteratorWithAncestors<'a, Node> 
    where Node: BorrowedTreeNode<'a> {
        
    pub (crate) fn new(root: &'a Node) -> BorrowedDFSPreorderIteratorWithAncestors<'a, Node> {
        BorrowedDFSPreorderIteratorWithAncestors { 
            root: Some(root),
            traversal_stack: Vec::new(),
            item_stack: Vec::new(),
        }
    }

    advance_dfs!(get_value_and_children_iter);
}

impl<'a, Node> StreamingIterator for BorrowedDFSPreorderIteratorWithAncestors<'a, Node> 
    where Node: BorrowedTreeNode<'a> {
    
    type Item = [Node::BorrowedValue];
    preorder_streaming_iterator_impl!();
}

impl<'a, Node> AncestorsIterator for BorrowedDFSPreorderIteratorWithAncestors<'a, Node> 
    where Node: BorrowedTreeNode<'a> {}


pub struct BorrowedBinaryDFSPreorderIterator<'a, Node>
    where Node: BorrowedBinaryTreeNode<'a> {

    root: Option<&'a Node>,
    traversal_stack: Vec<BinaryChildren<&'a Node>>,
}

impl<'a, Node> BorrowedBinaryDFSPreorderIterator<'a, Node> 
    where Node: BorrowedBinaryTreeNode<'a> {
        
    pub (crate) fn new(root: &'a Node) -> BorrowedBinaryDFSPreorderIterator<'a, Node> {
        BorrowedBinaryDFSPreorderIterator { 
            root: Some(root),
            traversal_stack: Vec::new()
        }
    }
}

impl<'a, Node> TreeIterator for BorrowedBinaryDFSPreorderIterator<'a, Node> 
    where Node: BorrowedBinaryTreeNode<'a> {
    fn leaves(self) -> impl LeavesIterator<Item = Self::Item> {
        BorrowedBinaryLeavesIterator {
            root: self.root,
            traversal_stack_bottom: self.traversal_stack,
            traversal_stack_top: Vec::new(),
            item_stack: Vec::new(),
        }
    }
    
    fn attach_ancestors(self) -> impl AncestorsIterator<Item = [Node::BorrowedValue]> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS preorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                BorrowedBinaryDFSPreorderIteratorWithAncestors::new(root)
            }
        }
    }
}

impl<'a, Node> Iterator for BorrowedBinaryDFSPreorderIterator<'a, Node> 
    where Node: BorrowedBinaryTreeNode<'a> {
    
    type Item = Node::BorrowedValue;
    dfs_preorder_next!(get_value_and_children_iter);
}

pub struct BorrowedBinaryDFSPreorderIteratorWithAncestors<'a, Node>
    where Node: BorrowedBinaryTreeNode<'a> {

    root: Option<&'a Node>,
    traversal_stack: Vec<BinaryChildren<&'a Node>>,
    item_stack: Vec<Node::BorrowedValue>,
}


impl<'a, Node> BorrowedBinaryDFSPreorderIteratorWithAncestors<'a, Node> 
    where Node: BorrowedBinaryTreeNode<'a> {
        
    pub (crate) fn new(root: &'a Node) -> BorrowedBinaryDFSPreorderIteratorWithAncestors<'a, Node> {
        BorrowedBinaryDFSPreorderIteratorWithAncestors { 
            root: Some(root),
            traversal_stack: Vec::new(),
            item_stack: Vec::new(),
        }
    }

    advance_dfs!(get_value_and_children_iter);
}

impl<'a, Node> StreamingIterator for BorrowedBinaryDFSPreorderIteratorWithAncestors<'a, Node> 
    where Node: BorrowedBinaryTreeNode<'a> {
    
    type Item = [Node::BorrowedValue];
    preorder_streaming_iterator_impl!();
}

impl<'a, Node> AncestorsIterator for BorrowedBinaryDFSPreorderIteratorWithAncestors<'a, Node> 
    where Node: BorrowedBinaryTreeNode<'a> {}
