use alloc::{collections::VecDeque, vec::Vec};
use streaming_iterator::{StreamingIterator, StreamingIteratorMut};

use super::{
    bfs_advance_iterator, bfs_next, bfs_streaming_iterator_impl, get_mut, TreeNodeVecDeque,
};
use crate::{
    leaves_iterators::{
        ancestors_breadth_first::mut_borrow::{
            MutBorrowedBFSLeavesIteratorWithAncestors,
            MutBorrowedBinaryBFSLeavesIteratorWithAncestors,
        },
        breadth_first::mut_borrow::{MutBorrowedBinaryLeavesIterator, MutBorrowedLeavesIterator},
    },
    prelude::{
        AncestorsIteratorMut, AncestorsLeavesIteratorMut, BinaryChildren, LeavesIterator,
        MutBorrowedBinaryTreeNode, MutBorrowedTreeNode, TreeIteratorMut,
    },
};

pub struct MutBorrowedBFSIterator<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    root: Option<&'a mut Node>,
    traversal_queue: VecDeque<Node::MutBorrowedChildren>,
}

impl<'a, Node> MutBorrowedBFSIterator<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    pub(crate) fn new(root: &'a mut Node) -> MutBorrowedBFSIterator<'a, Node> {
        MutBorrowedBFSIterator {
            root: Some(root),
            traversal_queue: VecDeque::new(),
        }
    }
}

impl<'a, Node> TreeIteratorMut for MutBorrowedBFSIterator<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    fn leaves(self) -> impl LeavesIterator<Item = Self::Item> {
        MutBorrowedLeavesIterator {
            root: self.root,
            old_traversal_queue: self.traversal_queue,
            new_traversal_queue: VecDeque::new(),
        }
    }

    fn attach_ancestors(self) -> impl AncestorsIteratorMut<Item = [Node::MutBorrowedValue]> {
        match self.root {
            None => panic!("Attempted to attach metadata to a BFS iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => MutBorrowedBFSIteratorWithAncestors::new(root)
        }
    }
}

impl<'a, Node> Iterator for MutBorrowedBFSIterator<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    type Item = Node::MutBorrowedValue;
    bfs_next!(get_value_and_children_iter_mut);
}

pub struct MutBorrowedBFSIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    pub(crate) is_root: bool,
    pub(crate) item_stack: Vec<Node::MutBorrowedValue>,
    pub(crate) tree_cache: TreeNodeVecDeque<Node::MutBorrowedValue>,
    pub(crate) traversal_stack: Vec<TreeNodeVecDeque<Node::MutBorrowedValue>>,
    pub(crate) iterator_queue: VecDeque<Node::MutBorrowedChildren>,
}

impl<'a, Node> MutBorrowedBFSIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    fn new(root: &'a mut Node) -> MutBorrowedBFSIteratorWithAncestors<'a, Node> {
        let (value, children) = root.get_value_and_children_iter_mut();
        let tree_cache = TreeNodeVecDeque {
            value: None,
            children: None,
        };
        let mut iterator_queue = VecDeque::new();
        let mut item_stack = Vec::new();

        item_stack.push(value);
        iterator_queue.push_back(children);

        MutBorrowedBFSIteratorWithAncestors {
            is_root: true,
            item_stack,
            iterator_queue,
            traversal_stack: Vec::new(),
            tree_cache,
        }
    }

    bfs_advance_iterator!(get_value_and_children_iter_mut);
}

impl<'a, Node> StreamingIterator for MutBorrowedBFSIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    type Item = [Node::MutBorrowedValue];

    bfs_streaming_iterator_impl!(get_value_and_children_iter_mut);
}

impl<'a, Node> StreamingIteratorMut for MutBorrowedBFSIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    get_mut!();
}

impl<'a, Node> AncestorsIteratorMut for MutBorrowedBFSIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    fn leaves(self) -> impl AncestorsLeavesIteratorMut<Item = Self::Item> {
        MutBorrowedBFSLeavesIteratorWithAncestors::new(self)
    }
}

pub struct MutBorrowedBinaryBFSIterator<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    root: Option<&'a mut Node>,
    traversal_queue: VecDeque<BinaryChildren<&'a mut Node>>,
}

impl<'a, Node> MutBorrowedBinaryBFSIterator<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    pub(crate) fn new(root: &'a mut Node) -> MutBorrowedBinaryBFSIterator<'a, Node> {
        MutBorrowedBinaryBFSIterator {
            root: Some(root),
            traversal_queue: VecDeque::new(),
        }
    }
}

impl<'a, Node> TreeIteratorMut for MutBorrowedBinaryBFSIterator<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    fn leaves(self) -> impl LeavesIterator<Item = Self::Item> {
        MutBorrowedBinaryLeavesIterator {
            root: self.root,
            old_traversal_queue: self.traversal_queue,
            new_traversal_queue: VecDeque::new(),
        }
    }

    fn attach_ancestors(self) -> impl AncestorsIteratorMut<Item = [Node::MutBorrowedValue]> {
        match self.root {
            None => panic!("Attempted to attach metadata to a BFS iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => MutBorrowedBinaryBFSIteratorWithAncestors::new(root)
        }
    }
}

impl<'a, Node> Iterator for MutBorrowedBinaryBFSIterator<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    type Item = Node::MutBorrowedValue;
    bfs_next!(get_value_and_children_iter_mut);
}

pub struct MutBorrowedBinaryBFSIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    pub(crate) is_root: bool,
    pub(crate) item_stack: Vec<Node::MutBorrowedValue>,
    pub(crate) tree_cache: TreeNodeVecDeque<Node::MutBorrowedValue>,
    pub(crate) traversal_stack: Vec<TreeNodeVecDeque<Node::MutBorrowedValue>>,
    pub(crate) iterator_queue: VecDeque<BinaryChildren<&'a mut Node>>,
}

impl<'a, Node> MutBorrowedBinaryBFSIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    fn new(root: &'a mut Node) -> MutBorrowedBinaryBFSIteratorWithAncestors<'a, Node> {
        let (value, children) = root.get_value_and_children_iter_mut();
        let tree_cache = TreeNodeVecDeque {
            value: None,
            children: None,
        };
        let mut iterator_queue = VecDeque::new();
        let mut item_stack = Vec::new();

        item_stack.push(value);
        iterator_queue.push_back(children);

        MutBorrowedBinaryBFSIteratorWithAncestors {
            is_root: true,
            item_stack,
            iterator_queue,
            traversal_stack: Vec::new(),
            tree_cache,
        }
    }

    bfs_advance_iterator!(get_value_and_children_iter_mut);
}

impl<'a, Node> StreamingIterator for MutBorrowedBinaryBFSIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    type Item = [Node::MutBorrowedValue];

    bfs_streaming_iterator_impl!(get_value_and_children_iter_mut);
}

impl<'a, Node> StreamingIteratorMut for MutBorrowedBinaryBFSIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    get_mut!();
}

impl<'a, Node> AncestorsIteratorMut for MutBorrowedBinaryBFSIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    fn leaves(self) -> impl AncestorsLeavesIteratorMut<Item = Self::Item> {
        MutBorrowedBinaryBFSLeavesIteratorWithAncestors::new(self)
    }
}
