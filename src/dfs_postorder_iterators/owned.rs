use crate::{
    leaves_iterators::{
        ancestors_depth_first::owned::{
            OwnedBinaryDFSLeavesPostorderIteratorWithAncestors,
            OwnedDFSLeavesPostorderIteratorWithAncestors,
        },
        depth_first::owned::{OwnedBinaryLeavesIterator, OwnedLeavesIterator},
    },
    prelude::{
        AncestorsIteratorMut, AncestorsLeavesIteratorMut, BinaryChildren, LeavesIterator,
        OwnedBinaryTreeNode, OwnedTreeNode, TreeIteratorMut,
    },
};
use alloc::vec::Vec;
use streaming_iterator::{StreamingIterator, StreamingIteratorMut};

use super::{dfs_postorder_next, get_mut, postorder_streaming_iterator_impl};

pub struct OwnedDFSPostorderIterator<Node>
where
    Node: OwnedTreeNode,
{
    root: Option<Node>,
    item_stack: Vec<Node::OwnedValue>,
    traversal_stack: Vec<Node::OwnedChildren>,
}

impl<Node> OwnedDFSPostorderIterator<Node>
where
    Node: OwnedTreeNode,
{
    pub(crate) fn new(root: Node) -> OwnedDFSPostorderIterator<Node> {
        OwnedDFSPostorderIterator {
            root: Some(root),
            item_stack: Vec::new(),
            traversal_stack: Vec::new(),
        }
    }
}

impl<Node> TreeIteratorMut for OwnedDFSPostorderIterator<Node>
where
    Node: OwnedTreeNode,
{
    fn leaves(self) -> impl LeavesIterator<Item = Self::Item> {
        OwnedLeavesIterator {
            root: self.root,
            traversal_stack_bottom: self.traversal_stack,
            traversal_stack_top: Vec::new(),
            item_stack: Vec::new(),
        }
    }

    fn attach_ancestors(self) -> impl AncestorsIteratorMut<Item = [Node::OwnedValue]> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS postorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                OwnedDFSPostorderIteratorWithAncestors::new(root)
            }
        }
    }
}

impl<Node> Iterator for OwnedDFSPostorderIterator<Node>
where
    Node: OwnedTreeNode,
{
    type Item = Node::OwnedValue;
    dfs_postorder_next!(get_value_and_children);
}

pub struct OwnedDFSPostorderIteratorWithAncestors<Node>
where
    Node: OwnedTreeNode,
{
    root: Option<Node>,
    item_stack: Vec<Node::OwnedValue>,
    traversal_stack: Vec<Node::OwnedChildren>,
}

impl<'a, Node> OwnedDFSPostorderIteratorWithAncestors<Node>
where
    Node: OwnedTreeNode,
{
    fn new(root: Node) -> OwnedDFSPostorderIteratorWithAncestors<Node> {
        Self {
            root: Some(root),
            item_stack: Vec::new(),
            traversal_stack: Vec::new(),
        }
    }
}

impl<Node> StreamingIterator for OwnedDFSPostorderIteratorWithAncestors<Node>
where
    Node: OwnedTreeNode,
{
    type Item = [Node::OwnedValue];
    postorder_streaming_iterator_impl!(get_value_and_children);
}

impl<Node> StreamingIteratorMut for OwnedDFSPostorderIteratorWithAncestors<Node>
where
    Node: OwnedTreeNode,
{
    get_mut!();
}

impl<Node> AncestorsIteratorMut for OwnedDFSPostorderIteratorWithAncestors<Node>
where
    Node: OwnedTreeNode,
{
    fn leaves(self) -> impl AncestorsLeavesIteratorMut<Item = Self::Item> {
        OwnedDFSLeavesPostorderIteratorWithAncestors {
            root: self.root,
            item_stack: self.item_stack,
            old_traversal_stack: self.traversal_stack.into_iter().collect(),
            new_traversal_stack: Vec::new(),
        }
    }
}

pub struct OwnedBinaryDFSPostorderIterator<Node>
where
    Node: OwnedBinaryTreeNode,
{
    root: Option<Node>,
    item_stack: Vec<Node::OwnedValue>,
    traversal_stack: Vec<BinaryChildren<Node>>,
}

impl<Node> OwnedBinaryDFSPostorderIterator<Node>
where
    Node: OwnedBinaryTreeNode,
{
    pub(crate) fn new(root: Node) -> OwnedBinaryDFSPostorderIterator<Node> {
        OwnedBinaryDFSPostorderIterator {
            root: Some(root),
            item_stack: Vec::new(),
            traversal_stack: Vec::new(),
        }
    }
}

impl<Node> TreeIteratorMut for OwnedBinaryDFSPostorderIterator<Node>
where
    Node: OwnedBinaryTreeNode,
{
    fn leaves(self) -> impl LeavesIterator<Item = Self::Item> {
        OwnedBinaryLeavesIterator {
            root: self.root,
            traversal_stack_bottom: self.traversal_stack,
            traversal_stack_top: Vec::new(),
            item_stack: Vec::new(),
        }
    }

    fn attach_ancestors(self) -> impl AncestorsIteratorMut<Item = [Node::OwnedValue]> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS postorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                OwnedBinaryDFSPostorderIteratorWithAncestors::new(root)
            }
        }
    }
}

impl<Node> Iterator for OwnedBinaryDFSPostorderIterator<Node>
where
    Node: OwnedBinaryTreeNode,
{
    type Item = Node::OwnedValue;
    dfs_postorder_next!(get_value_and_children);
}

pub struct OwnedBinaryDFSPostorderIteratorWithAncestors<Node>
where
    Node: OwnedBinaryTreeNode,
{
    root: Option<Node>,
    item_stack: Vec<Node::OwnedValue>,
    traversal_stack: Vec<BinaryChildren<Node>>,
}

impl<'a, Node> OwnedBinaryDFSPostorderIteratorWithAncestors<Node>
where
    Node: OwnedBinaryTreeNode,
{
    fn new(root: Node) -> OwnedBinaryDFSPostorderIteratorWithAncestors<Node> {
        Self {
            root: Some(root),
            item_stack: Vec::new(),
            traversal_stack: Vec::new(),
        }
    }
}

impl<Node> StreamingIterator for OwnedBinaryDFSPostorderIteratorWithAncestors<Node>
where
    Node: OwnedBinaryTreeNode,
{
    type Item = [Node::OwnedValue];
    postorder_streaming_iterator_impl!(get_value_and_children);
}

impl<Node> StreamingIteratorMut for OwnedBinaryDFSPostorderIteratorWithAncestors<Node>
where
    Node: OwnedBinaryTreeNode,
{
    get_mut!();
}

impl<Node> AncestorsIteratorMut for OwnedBinaryDFSPostorderIteratorWithAncestors<Node>
where
    Node: OwnedBinaryTreeNode,
{
    fn leaves(self) -> impl AncestorsLeavesIteratorMut<Item = Self::Item> {
        OwnedBinaryDFSLeavesPostorderIteratorWithAncestors {
            root: self.root,
            item_stack: self.item_stack,
            old_traversal_stack: self.traversal_stack.into_iter().collect(),
            new_traversal_stack: Vec::new(),
        }
    }
}
