use alloc::vec::Vec;
use streaming_iterator::{
    StreamingIterator,
    StreamingIteratorMut
};

use crate::{
    prelude::{
        OwnedTreeNode, 
        BinaryChildren, 
        OwnedBinaryTreeNode, 
        LeavesIterator, 
        AncestorsIteratorMut, 
        AncestorsIterator, 
        TreeIteratorMut
    }, 
    leaves_iterators::depth_first::owned::{
        OwnedLeavesIterator, 
        OwnedBinaryLeavesIterator
    }
};

use super::{dfs_preorder_next, preorder_streaming_iterator_impl, advance_dfs, get_mut};

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

impl<Node> TreeIteratorMut for OwnedDFSPreorderIterator<Node> 
    where Node: OwnedTreeNode {

    fn leaves(self) -> impl LeavesIterator<Item = Self::Item> {
        OwnedLeavesIterator {
            root: self.root,
            traversal_stack_bottom: self.traversal_stack,
            traversal_stack_top: Vec::new(),
            item_stack: Vec::new()
        }
    }

    fn attach_ancestors(self) -> impl AncestorsIteratorMut<Item = [Node::OwnedValue]> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS preorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                OwnedDFSPreorderIteratorWithAncestors::new(root)
            }
        }
    }
}

impl<Node> Iterator for OwnedDFSPreorderIterator<Node> 
    where Node: OwnedTreeNode {
    
    type Item = Node::OwnedValue;
    dfs_preorder_next!(get_value_and_children);
}

pub struct OwnedDFSPreorderIteratorWithAncestors<Node>
    where Node: OwnedTreeNode {

    root: Option<Node>,
    traversal_stack: Vec<Node::OwnedChildren>,
    item_stack: Vec<Node::OwnedValue>,
}


impl<'a, Node> OwnedDFSPreorderIteratorWithAncestors<Node> 
    where Node: OwnedTreeNode {
        
    pub (crate) fn new(root: Node) -> OwnedDFSPreorderIteratorWithAncestors<Node> {
        OwnedDFSPreorderIteratorWithAncestors { 
            root: Some(root),
            traversal_stack: Vec::new(),
            item_stack: Vec::new(),
        }
    }
    
    advance_dfs!(get_value_and_children);
}

impl<Node> StreamingIterator for OwnedDFSPreorderIteratorWithAncestors<Node> 
    where Node: OwnedTreeNode {
    
    type Item = [Node::OwnedValue];
    preorder_streaming_iterator_impl!();
}

impl<Node> StreamingIteratorMut for OwnedDFSPreorderIteratorWithAncestors<Node> 
    where Node: OwnedTreeNode {

    get_mut!();
}

impl<Node> AncestorsIterator for OwnedDFSPreorderIteratorWithAncestors<Node> 
    where Node: OwnedTreeNode {}

impl<Node> AncestorsIteratorMut for OwnedDFSPreorderIteratorWithAncestors<Node> 
    where Node: OwnedTreeNode {}

pub struct OwnedBinaryDFSPreorderIterator<Node>
    where Node: OwnedBinaryTreeNode {

    root: Option<Node>,
    traversal_stack: Vec<BinaryChildren<Node>>,
}

impl<Node> OwnedBinaryDFSPreorderIterator<Node> 
    where Node: OwnedBinaryTreeNode {
        
    pub (crate) fn new(root: Node) -> OwnedBinaryDFSPreorderIterator<Node> {
        OwnedBinaryDFSPreorderIterator { 
            root: Some(root),
            traversal_stack: Vec::new()
        }
    }
}

impl<Node> TreeIteratorMut for OwnedBinaryDFSPreorderIterator<Node> 
    where Node: OwnedBinaryTreeNode {

    fn leaves(self) -> impl LeavesIterator<Item = Self::Item> {
        OwnedBinaryLeavesIterator {
            root: self.root,
            traversal_stack_bottom: self.traversal_stack,
            traversal_stack_top: Vec::new(),
            item_stack: Vec::new(),
        }
    }

    fn attach_ancestors(self) -> impl AncestorsIteratorMut<Item = [Self::Item]> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS preorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                OwnedBinaryDFSPreorderIteratorWithAncestors::new(root)
            }
        }
    }
}

impl<Node> Iterator for OwnedBinaryDFSPreorderIterator<Node> 
    where Node: OwnedBinaryTreeNode {
    
    type Item = Node::OwnedValue;
    dfs_preorder_next!(get_value_and_children);
}

pub struct OwnedBinaryDFSPreorderIteratorWithAncestors<Node>
    where Node: OwnedBinaryTreeNode {

    root: Option<Node>,
    traversal_stack: Vec<BinaryChildren<Node>>,
    item_stack: Vec<Node::OwnedValue>,
}


impl<'a, Node> OwnedBinaryDFSPreorderIteratorWithAncestors<Node> 
    where Node: OwnedBinaryTreeNode {
        
    pub (crate) fn new(root: Node) -> OwnedBinaryDFSPreorderIteratorWithAncestors<Node> {
        OwnedBinaryDFSPreorderIteratorWithAncestors { 
            root: Some(root),
            traversal_stack: Vec::new(),
            item_stack: Vec::new(),
        }
    }
    
    advance_dfs!(get_value_and_children);
}

impl<'a, Node> StreamingIterator for OwnedBinaryDFSPreorderIteratorWithAncestors<Node> 
    where Node: OwnedBinaryTreeNode {
    
    type Item = [Node::OwnedValue];
    preorder_streaming_iterator_impl!();
}

impl<'a, Node> StreamingIteratorMut for OwnedBinaryDFSPreorderIteratorWithAncestors<Node> 
    where Node: OwnedBinaryTreeNode {
    
    get_mut!();
}

impl<Node> AncestorsIterator for OwnedBinaryDFSPreorderIteratorWithAncestors<Node>
    where Node: OwnedBinaryTreeNode {}

impl<Node> AncestorsIteratorMut for OwnedBinaryDFSPreorderIteratorWithAncestors<Node>
    where Node: OwnedBinaryTreeNode {}