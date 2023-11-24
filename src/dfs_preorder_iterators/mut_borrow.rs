use alloc::vec::Vec;
use streaming_iterator::{StreamingIterator, StreamingIteratorMut};

use crate::{
    leaves_iterators::{
        ancestors_depth_first::mut_borrow::{
            MutBorrowedBinaryDFSLeavesPostorderIteratorWithAncestors,
            MutBorrowedDFSLeavesPostorderIteratorWithAncestors,
        },
        depth_first::mut_borrow::{MutBorrowedBinaryLeavesIterator, MutBorrowedLeavesIterator},
    },
    prelude::{
        AncestorsIteratorMut, AncestorsLeavesIteratorMut, BinaryChildren, LeavesIterator,
        MutBorrowedBinaryTreeNode, MutBorrowedTreeNode, TreeIteratorMut,
    },
};

use super::{advance_dfs, dfs_preorder_next, get_mut, preorder_streaming_iterator_impl};

pub struct MutBorrowedDFSPreorderIterator<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    root: Option<&'a mut Node>,
    traversal_stack: Vec<Node::MutBorrowedChildren>,
}

impl<'a, Node> MutBorrowedDFSPreorderIterator<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    pub(crate) fn new(root: &'a mut Node) -> MutBorrowedDFSPreorderIterator<'a, Node> {
        MutBorrowedDFSPreorderIterator {
            root: Some(root),
            traversal_stack: Vec::new(),
        }
    }
}

impl<'a, Node> TreeIteratorMut for MutBorrowedDFSPreorderIterator<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    fn leaves(self) -> impl LeavesIterator<Item = Self::Item> {
        MutBorrowedLeavesIterator {
            root: self.root,
            traversal_stack_bottom: self.traversal_stack,
            traversal_stack_top: Vec::new(),
            item_stack: Vec::new(),
        }
    }

    fn attach_ancestors(self) -> impl AncestorsIteratorMut<Item = [Node::MutBorrowedValue]> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS preorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                MutBorrowedDFSPreorderIteratorWithAncestors::new(root)
            }
        }
    }
}

impl<'a, Node> Iterator for MutBorrowedDFSPreorderIterator<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    type Item = Node::MutBorrowedValue;
    dfs_preorder_next!(get_value_and_children_iter_mut);
}

pub struct MutBorrowedDFSPreorderIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    root: Option<&'a mut Node>,
    traversal_stack: Vec<Node::MutBorrowedChildren>,
    item_stack: Vec<Node::MutBorrowedValue>,
}

impl<'a, Node> MutBorrowedDFSPreorderIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    pub(crate) fn new(root: &'a mut Node) -> MutBorrowedDFSPreorderIteratorWithAncestors<'a, Node> {
        MutBorrowedDFSPreorderIteratorWithAncestors {
            root: Some(root),
            traversal_stack: Vec::new(),
            item_stack: Vec::new(),
        }
    }

    advance_dfs!(get_value_and_children_iter_mut);
}

impl<'a, Node> StreamingIterator for MutBorrowedDFSPreorderIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    type Item = [Node::MutBorrowedValue];
    preorder_streaming_iterator_impl!();
}

impl<'a, Node> StreamingIteratorMut for MutBorrowedDFSPreorderIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    get_mut!();
}

impl<'a, Node> AncestorsIteratorMut for MutBorrowedDFSPreorderIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    fn leaves(self) -> impl AncestorsLeavesIteratorMut<Item = Self::Item> {
        MutBorrowedDFSLeavesPostorderIteratorWithAncestors {
            root: self.root,
            item_stack: self.item_stack,
            old_traversal_stack: self.traversal_stack,
            new_traversal_stack: Vec::new(),
        }
    }
}

pub struct MutBorrowedBinaryDFSPreorderIterator<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    root: Option<&'a mut Node>,
    traversal_stack: Vec<BinaryChildren<&'a mut Node>>,
}

impl<'a, Node> MutBorrowedBinaryDFSPreorderIterator<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    pub(crate) fn new(root: &'a mut Node) -> MutBorrowedBinaryDFSPreorderIterator<'a, Node> {
        MutBorrowedBinaryDFSPreorderIterator {
            root: Some(root),
            traversal_stack: Vec::new(),
        }
    }
}

impl<'a, Node> TreeIteratorMut for MutBorrowedBinaryDFSPreorderIterator<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    fn leaves(self) -> impl LeavesIterator<Item = Self::Item> {
        MutBorrowedBinaryLeavesIterator {
            root: self.root,
            traversal_stack_bottom: self.traversal_stack,
            traversal_stack_top: Vec::new(),
            item_stack: Vec::new(),
        }
    }

    fn attach_ancestors(self) -> impl AncestorsIteratorMut<Item = [Node::MutBorrowedValue]> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS preorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                MutBorrowedBinaryDFSPreorderIteratorWithAncestors::new(root)
            }
        }
    }
}

impl<'a, Node> Iterator for MutBorrowedBinaryDFSPreorderIterator<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    type Item = Node::MutBorrowedValue;
    dfs_preorder_next!(get_value_and_children_iter_mut);
}

pub struct MutBorrowedBinaryDFSPreorderIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    root: Option<&'a mut Node>,
    traversal_stack: Vec<BinaryChildren<&'a mut Node>>,
    item_stack: Vec<Node::MutBorrowedValue>,
}

impl<'a, Node> MutBorrowedBinaryDFSPreorderIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    pub(crate) fn new(
        root: &'a mut Node,
    ) -> MutBorrowedBinaryDFSPreorderIteratorWithAncestors<'a, Node> {
        MutBorrowedBinaryDFSPreorderIteratorWithAncestors {
            root: Some(root),
            traversal_stack: Vec::new(),
            item_stack: Vec::new(),
        }
    }

    advance_dfs!(get_value_and_children_iter_mut);
}

impl<'a, Node> StreamingIterator for MutBorrowedBinaryDFSPreorderIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    type Item = [Node::MutBorrowedValue];
    preorder_streaming_iterator_impl!();
}

impl<'a, Node> StreamingIteratorMut for MutBorrowedBinaryDFSPreorderIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    get_mut!();
}

impl<'a, Node> AncestorsIteratorMut for MutBorrowedBinaryDFSPreorderIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    fn leaves(self) -> impl AncestorsLeavesIteratorMut<Item = Self::Item> {
        MutBorrowedBinaryDFSLeavesPostorderIteratorWithAncestors {
            root: self.root,
            item_stack: self.item_stack,
            old_traversal_stack: self.traversal_stack,
            new_traversal_stack: Vec::new(),
        }
    }
}
