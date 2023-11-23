use alloc::vec::Vec;
use streaming_iterator::StreamingIterator;
use crate::{
    prelude::{
        BorrowedTreeNode, 
        BinaryChildren, 
        BorrowedBinaryTreeNode, TreeIterator, LeavesIterator, AncestorsIterator
    }, 
    leaves_iterators::depth_first::borrow::{
        BorrowedLeavesIterator, 
        BorrowedBinaryLeavesIterator
    }
};

use super::{
    postorder_streaming_iterator_impl, 
    dfs_postorder_next
};

pub struct BorrowedDFSPostorderIterator<'a, Node> 
    where Node: BorrowedTreeNode<'a> {

    root: Option<&'a Node>,
    item_stack: Vec<Node::BorrowedValue>,
    traversal_stack: Vec<Node::BorrowedChildren>
}

impl<'a, Node> BorrowedDFSPostorderIterator<'a, Node> 
    where Node: BorrowedTreeNode<'a> {

    pub (crate) fn new(root: &'a Node) -> BorrowedDFSPostorderIterator<'a, Node> {
        BorrowedDFSPostorderIterator { 
            root: Some(root),
            item_stack: Vec::new(), 
            traversal_stack: Vec::new() 
        }
    }
}

impl<'a, Node> TreeIterator for BorrowedDFSPostorderIterator<'a, Node> 
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
            None => panic!("Attempted to attach metadata to a DFS postorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                BorrowedDFSPostorderIteratorWithAncestors::new(root)
            }
        }
    }
}

impl<'a, Node> Iterator for BorrowedDFSPostorderIterator<'a, Node> 
    where Node: BorrowedTreeNode<'a> {

    type Item = Node::BorrowedValue;
    dfs_postorder_next!(get_value_and_children_iter);
}

pub struct BorrowedDFSPostorderIteratorWithAncestors<'a, Node>
    where Node: BorrowedTreeNode<'a> {

    root: Option<&'a Node>,
    item_stack: Vec<Node::BorrowedValue>,
    traversal_stack: Vec<Node::BorrowedChildren>,
}

impl<'a, Node> BorrowedDFSPostorderIteratorWithAncestors<'a, Node> 
    where Node: BorrowedTreeNode<'a> {

    fn new(root: &'a Node) -> BorrowedDFSPostorderIteratorWithAncestors<'_, Node> {
        Self {
            root: Some(root),
            item_stack: Vec::new(),
            traversal_stack: Vec::new(),
        }
    }
}

impl<'a, Node> StreamingIterator for BorrowedDFSPostorderIteratorWithAncestors<'a, Node> 
    where Node: BorrowedTreeNode<'a> {

    type Item = [Node::BorrowedValue];
    postorder_streaming_iterator_impl!(get_value_and_children_iter);
}

impl<'a, Node> AncestorsIterator for BorrowedDFSPostorderIteratorWithAncestors<'a, Node> 
    where Node: BorrowedTreeNode<'a> {}

pub struct BorrowedBinaryDFSPostorderIterator<'a, Node> 
    where Node: BorrowedBinaryTreeNode<'a> {

    root: Option<&'a Node>,
    item_stack: Vec<Node::BorrowedValue>,
    traversal_stack: Vec<BinaryChildren<&'a Node>>
}

impl<'a, Node> BorrowedBinaryDFSPostorderIterator<'a, Node> 
    where Node: BorrowedBinaryTreeNode<'a> {

    pub (crate) fn new(root: &'a Node) -> BorrowedBinaryDFSPostorderIterator<'a, Node> {
        BorrowedBinaryDFSPostorderIterator { 
            root: Some(root),
            item_stack: Vec::new(), 
            traversal_stack: Vec::new() 
        }
    }
}

impl<'a, Node> TreeIterator for BorrowedBinaryDFSPostorderIterator<'a, Node> 
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
            None => panic!("Attempted to attach metadata to a DFS postorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                BorrowedBinaryDFSPostorderIteratorWithAncestors::new(root)
            }
        }
    }
}

impl<'a, Node> Iterator for BorrowedBinaryDFSPostorderIterator<'a, Node> 
    where Node: BorrowedBinaryTreeNode<'a> {

    type Item = Node::BorrowedValue;
    dfs_postorder_next!(get_value_and_children_iter);
}

pub struct BorrowedBinaryDFSPostorderIteratorWithAncestors<'a, Node>
    where Node: BorrowedBinaryTreeNode<'a> {

    root: Option<&'a Node>,
    item_stack: Vec<Node::BorrowedValue>,
    traversal_stack: Vec<BinaryChildren<&'a Node>>,
}

impl<'a, Node> BorrowedBinaryDFSPostorderIteratorWithAncestors<'a, Node> 
    where Node: BorrowedBinaryTreeNode<'a> {

    fn new(root: &'a Node) -> BorrowedBinaryDFSPostorderIteratorWithAncestors<'_, Node> {
        Self {
            root: Some(root),
            item_stack: Vec::new(),
            traversal_stack: Vec::new(),
        }
    }
}

impl<'a, Node> StreamingIterator for BorrowedBinaryDFSPostorderIteratorWithAncestors<'a, Node> 
    where Node: BorrowedBinaryTreeNode<'a> {

    type Item = [Node::BorrowedValue];
    postorder_streaming_iterator_impl!(get_value_and_children_iter);
}

impl<'a, Node> AncestorsIterator for BorrowedBinaryDFSPostorderIteratorWithAncestors<'a, Node> 
    where Node: BorrowedBinaryTreeNode<'a> {}