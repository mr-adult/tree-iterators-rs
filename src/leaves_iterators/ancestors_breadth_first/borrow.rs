use alloc::{collections::VecDeque, vec::Vec};
use core::iter::Peekable;
use streaming_iterator::StreamingIterator;

use crate::{
    bfs_iterators::borrow::{
        BorrowedBFSIteratorWithAncestors, BorrowedBinaryBFSIteratorWithAncestors,
    },
    prelude::{BinaryChildren, BorrowedBinaryTreeNode, BorrowedTreeNode},
};

use super::{bfs_next, streaming_leaves};

use crate::bfs_iterators::TreeNodeVecDeque;

pub struct BorrowedBFSLeavesIteratorWithAncestors<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    is_root: bool,
    item_stack: Vec<Node::BorrowedValue>,
    tree_cache: TreeNodeVecDeque<Node::BorrowedValue>,
    traversal_stack: Vec<TreeNodeVecDeque<Node::BorrowedValue>>,
    iterator_queue: VecDeque<Peekable<<Node::BorrowedChildren as IntoIterator>::IntoIter>>,
}

impl<'a, Node> BorrowedBFSLeavesIteratorWithAncestors<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    pub(crate) fn new(
        source: BorrowedBFSIteratorWithAncestors<'a, Node>,
    ) -> BorrowedBFSLeavesIteratorWithAncestors<'a, Node> {
        BorrowedBFSLeavesIteratorWithAncestors {
            is_root: source.is_root,
            item_stack: source.item_stack,
            iterator_queue: source
                .iterator_queue
                .into_iter()
                .map(|val| val.peekable())
                .collect(),
            traversal_stack: source.traversal_stack,
            tree_cache: source.tree_cache,
        }
    }

    bfs_next!(get_value_and_children_iter);
}

impl<'a, Node> StreamingIterator for BorrowedBFSLeavesIteratorWithAncestors<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    type Item = [Node::BorrowedValue];

    streaming_leaves!();
}

pub struct BorrowedBinaryBFSLeavesIteratorWithAncestors<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    is_root: bool,
    item_stack: Vec<Node::BorrowedValue>,
    tree_cache: TreeNodeVecDeque<Node::BorrowedValue>,
    traversal_stack: Vec<TreeNodeVecDeque<Node::BorrowedValue>>,
    iterator_queue: VecDeque<Peekable<BinaryChildren<&'a Node>>>,
}

impl<'a, Node> BorrowedBinaryBFSLeavesIteratorWithAncestors<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    pub(crate) fn new(
        source: BorrowedBinaryBFSIteratorWithAncestors<'a, Node>,
    ) -> BorrowedBinaryBFSLeavesIteratorWithAncestors<'a, Node> {
        BorrowedBinaryBFSLeavesIteratorWithAncestors {
            is_root: source.is_root,
            item_stack: source.item_stack,
            iterator_queue: source
                .iterator_queue
                .into_iter()
                .map(|val| val.peekable())
                .collect(),
            traversal_stack: source.traversal_stack,
            tree_cache: source.tree_cache,
        }
    }

    bfs_next!(get_value_and_children_iter);
}

impl<'a, Node> StreamingIterator for BorrowedBinaryBFSLeavesIteratorWithAncestors<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    type Item = [Node::BorrowedValue];

    streaming_leaves!();
}
