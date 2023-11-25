use alloc::{collections::VecDeque, vec::Vec};
use core::iter::Peekable;
use streaming_iterator::{StreamingIterator, StreamingIteratorMut};

use crate::{
    bfs_iterators::mut_borrow::{
        MutBorrowedBFSIteratorWithAncestors, MutBorrowedBinaryBFSIteratorWithAncestors,
    },
    prelude::{BinaryChildren, MutBorrowedBinaryTreeNode, MutBorrowedTreeNode},
};

use super::{bfs_next, get_mut, streaming_leaves};

use crate::bfs_iterators::TreeNodeVecDeque;

pub struct MutBorrowedBFSLeavesIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    is_root: bool,
    item_stack: Vec<Node::MutBorrowedValue>,
    tree_cache: TreeNodeVecDeque<Node::MutBorrowedValue>,
    traversal_stack: Vec<TreeNodeVecDeque<Node::MutBorrowedValue>>,
    iterator_queue: VecDeque<Option<Peekable<Node::MutBorrowedChildren>>>,
}

impl<'a, Node> MutBorrowedBFSLeavesIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    pub(crate) fn new(
        source: MutBorrowedBFSIteratorWithAncestors<'a, Node>,
    ) -> MutBorrowedBFSLeavesIteratorWithAncestors<'a, Node> {
        MutBorrowedBFSLeavesIteratorWithAncestors {
            is_root: source.is_root,
            item_stack: source.item_stack,
            iterator_queue: source
                .iterator_queue
                .into_iter()
                .map(|val| match val {
                    None => None,
                    Some(val) => Some(val.peekable()),
                })
                .collect(),
            traversal_stack: source.traversal_stack,
            tree_cache: source.tree_cache,
        }
    }

    bfs_next!(get_value_and_children_iter_mut);
}

impl<'a, Node> StreamingIterator for MutBorrowedBFSLeavesIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    type Item = [Node::MutBorrowedValue];

    streaming_leaves!();
}

impl<'a, Node> StreamingIteratorMut for MutBorrowedBFSLeavesIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    get_mut!();
}

pub struct MutBorrowedBinaryBFSLeavesIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    is_root: bool,
    item_stack: Vec<Node::MutBorrowedValue>,
    tree_cache: TreeNodeVecDeque<Node::MutBorrowedValue>,
    traversal_stack: Vec<TreeNodeVecDeque<Node::MutBorrowedValue>>,
    iterator_queue: VecDeque<Option<Peekable<BinaryChildren<&'a mut Node>>>>,
}

impl<'a, Node> MutBorrowedBinaryBFSLeavesIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    pub(crate) fn new(
        source: MutBorrowedBinaryBFSIteratorWithAncestors<'a, Node>,
    ) -> MutBorrowedBinaryBFSLeavesIteratorWithAncestors<'a, Node> {
        MutBorrowedBinaryBFSLeavesIteratorWithAncestors {
            is_root: source.is_root,
            item_stack: source.item_stack,
            iterator_queue: source
                .iterator_queue
                .into_iter()
                .map(|val| match val {
                    None => None,
                    Some(val) => Some(val.peekable()),
                })
                .collect(),
            traversal_stack: source.traversal_stack,
            tree_cache: source.tree_cache,
        }
    }

    bfs_next!(get_value_and_children_iter_mut);
}

impl<'a, Node> StreamingIterator for MutBorrowedBinaryBFSLeavesIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    type Item = [Node::MutBorrowedValue];

    streaming_leaves!();
}

impl<'a, Node> StreamingIteratorMut for MutBorrowedBinaryBFSLeavesIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    get_mut!();
}
