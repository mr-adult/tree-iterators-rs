use alloc::vec::Vec;
use streaming_iterator::{
    StreamingIterator,
    StreamingIteratorMut
};
use crate::{
    prelude::{
        MutBorrowedTreeNode, 
        BinaryChildren, 
        MutBorrowedBinaryTreeNode, 
        LeavesIterator, 
        TreeIteratorMut, 
        AncestorsIterator, 
        AncestorsIteratorMut
    }, 
    leaves_iterators::depth_first::mut_borrow::{
        MutBorrowedLeavesIterator, 
        MutBorrowedBinaryLeavesIterator
    }
};

use super::{
    get_mut,
    dfs_postorder_next, 
    postorder_streaming_iterator_impl
};

pub struct MutBorrowedDFSPostorderIterator<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {

    root: Option<&'a mut Node>,
    item_stack: Vec<Node::MutBorrowedValue>,
    traversal_stack: Vec<Node::MutBorrowedChildren>
}

impl<'a, Node> MutBorrowedDFSPostorderIterator<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {

    pub (crate) fn new(root: &'a mut Node) -> MutBorrowedDFSPostorderIterator<'a, Node> {
        MutBorrowedDFSPostorderIterator { 
            root: Some(root),
            item_stack: Vec::new(), 
            traversal_stack: Vec::new() 
        }
    }
}

impl<'a, Node> TreeIteratorMut for MutBorrowedDFSPostorderIterator<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {

    fn leaves(self) -> impl LeavesIterator<Item = Self::Item> {
        MutBorrowedLeavesIterator { 
            root: self.root,
            traversal_stack_bottom: self.traversal_stack,
            traversal_stack_top: Vec::new(),
            item_stack: Vec::new()
        }
    }

    fn attach_ancestors(self) -> impl AncestorsIteratorMut<Item = [Node::MutBorrowedValue]> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS postorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                MutBorrowedDFSPostorderIteratorWithAncestors::new(root)
            }
        }
    }
}

impl<'a, Node> Iterator for MutBorrowedDFSPostorderIterator<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {

    type Item = Node::MutBorrowedValue;
    dfs_postorder_next!(get_value_and_children_iter_mut);
}

pub struct MutBorrowedDFSPostorderIteratorWithAncestors<'a, Node>
    where Node: MutBorrowedTreeNode<'a> {

    root: Option<&'a mut Node>,
    item_stack: Vec<Node::MutBorrowedValue>,
    traversal_stack: Vec<Node::MutBorrowedChildren>,
}

impl<'a, Node> MutBorrowedDFSPostorderIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {

    fn new(root: &'a mut Node) -> MutBorrowedDFSPostorderIteratorWithAncestors<'_, Node> {
        Self {
            root: Some(root),
            item_stack: Vec::new(),
            traversal_stack: Vec::new(),
        }
    }
}

impl<'a, Node> StreamingIterator for MutBorrowedDFSPostorderIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {

    type Item = [Node::MutBorrowedValue];
    postorder_streaming_iterator_impl!(get_value_and_children_iter_mut);
}

impl<'a, Node> StreamingIteratorMut for MutBorrowedDFSPostorderIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {

    get_mut!();
}

impl<'a, Node> AncestorsIterator for MutBorrowedDFSPostorderIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {}

impl<'a, Node> AncestorsIteratorMut for MutBorrowedDFSPostorderIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {}

pub struct MutBorrowedBinaryDFSPostorderIterator<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {

    root: Option<&'a mut Node>,
    item_stack: Vec<Node::MutBorrowedValue>,
    traversal_stack: Vec<BinaryChildren<&'a mut Node>>
}

impl<'a, Node> MutBorrowedBinaryDFSPostorderIterator<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {

    pub (crate) fn new(root: &'a mut Node) -> MutBorrowedBinaryDFSPostorderIterator<'a, Node> {
        MutBorrowedBinaryDFSPostorderIterator { 
            root: Some(root),
            item_stack: Vec::new(), 
            traversal_stack: Vec::new() 
        }
    }
}

impl<'a, Node> TreeIteratorMut for MutBorrowedBinaryDFSPostorderIterator<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {

    fn leaves(self) -> impl LeavesIterator<Item = Self::Item> {
        MutBorrowedBinaryLeavesIterator { 
            root: self.root, 
            traversal_stack_bottom: self.traversal_stack,
            traversal_stack_top: Vec::new(),
            item_stack: Vec::new()
        }
    }

    fn attach_ancestors(self) -> impl AncestorsIteratorMut<Item = [Node::MutBorrowedValue]> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS postorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                MutBorrowedBinaryDFSPostorderIteratorWithAncestors::new(root)
            }
        }
    }
}

impl<'a, Node> Iterator for MutBorrowedBinaryDFSPostorderIterator<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {

    type Item = Node::MutBorrowedValue;
    dfs_postorder_next!(get_value_and_children_iter_mut);
}

pub struct MutBorrowedBinaryDFSPostorderIteratorWithAncestors<'a, Node>
    where Node: MutBorrowedBinaryTreeNode<'a> {

    root: Option<&'a mut Node>,
    item_stack: Vec<Node::MutBorrowedValue>,
    traversal_stack: Vec<BinaryChildren<&'a mut Node>>,
}

impl<'a, Node> MutBorrowedBinaryDFSPostorderIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {

    fn new(root: &'a mut Node) -> MutBorrowedBinaryDFSPostorderIteratorWithAncestors<'_, Node> {
        Self {
            root: Some(root),
            item_stack: Vec::new(),
            traversal_stack: Vec::new(),
        }
    }
}

impl<'a, Node> StreamingIterator for MutBorrowedBinaryDFSPostorderIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {

    type Item = [Node::MutBorrowedValue];
    postorder_streaming_iterator_impl!(get_value_and_children_iter_mut);
}

impl<'a, Node> StreamingIteratorMut for MutBorrowedBinaryDFSPostorderIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {

    get_mut!();
}

impl<'a, Node> AncestorsIterator for MutBorrowedBinaryDFSPostorderIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {}

impl<'a, Node> AncestorsIteratorMut for MutBorrowedBinaryDFSPostorderIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedBinaryTreeNode<'a> {}