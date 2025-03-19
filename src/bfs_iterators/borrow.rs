use alloc::{collections::VecDeque, vec::Vec};
use streaming_iterator::StreamingIterator;

use crate::{
    leaves_iterators::{
        ancestors_breadth_first::borrow::{
            BorrowedBFSLeavesIteratorWithAncestors, BorrowedBinaryBFSLeavesIteratorWithAncestors,
        },
        breadth_first::borrow::{BorrowedBinaryLeavesIterator, BorrowedLeavesIterator},
    },
    prelude::{BinaryChildren, BorrowedBinaryTreeNode, BorrowedTreeNode},
};

use super::{bfs_advance_iterator, bfs_next, bfs_streaming_iterator_impl, TreeNodeVecDeque};

pub struct BorrowedBFSIterator<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    root: Option<&'a Node>,
    traversal_queue: VecDeque<<Node::BorrowedChildren as IntoIterator>::IntoIter>,
}

impl<'a, Node> BorrowedBFSIterator<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    pub(crate) fn new(root: &'a Node) -> BorrowedBFSIterator<'a, Node> {
        BorrowedBFSIterator {
            root: Some(root),
            traversal_queue: VecDeque::new(),
        }
    }

    #[doc = include_str!("../../doc_files/leaves.md")]
    pub fn leaves(self) -> BorrowedLeavesIterator<'a, Node> {
        BorrowedLeavesIterator {
            root: self.root,
            old_traversal_queue: self.traversal_queue,
            new_traversal_queue: VecDeque::new(),
        }
    }

    #[doc = include_str!("../../doc_files/attach_ancestors.md")]
    pub fn attach_ancestors(self) -> BorrowedBFSIteratorWithAncestors<'a, Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a BFS iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => BorrowedBFSIteratorWithAncestors::new(root)
        }
    }
}

impl<'a, Node> Iterator for BorrowedBFSIterator<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    type Item = Node::BorrowedValue;
    bfs_next!(get_value_and_children_iter);
}

pub struct BorrowedBFSIteratorWithAncestors<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    pub(crate) is_root: bool,
    pub(crate) item_stack: Vec<Node::BorrowedValue>,
    pub(crate) tree_cache: TreeNodeVecDeque<Node::BorrowedValue>,
    pub(crate) traversal_stack: Vec<TreeNodeVecDeque<Node::BorrowedValue>>,
    pub(crate) iterator_queue: VecDeque<<Node::BorrowedChildren as IntoIterator>::IntoIter>,
}

impl<'a, Node> BorrowedBFSIteratorWithAncestors<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    fn new(root: &'a Node) -> BorrowedBFSIteratorWithAncestors<'a, Node> {
        let (value, children) = root.get_value_and_children_iter();
        let tree_cache = TreeNodeVecDeque {
            value: None,
            children: None,
        };
        let mut iterator_queue = VecDeque::new();
        let mut item_stack = Vec::new();

        item_stack.push(value);
        iterator_queue.push_back(children.into_iter());

        BorrowedBFSIteratorWithAncestors {
            is_root: true,
            item_stack,
            iterator_queue,
            traversal_stack: Vec::new(),
            tree_cache,
        }
    }

    #[doc = include_str!("../../doc_files/ancestors_leaves.md")]
    pub fn leaves(self) -> BorrowedBFSLeavesIteratorWithAncestors<'a, Node> {
        BorrowedBFSLeavesIteratorWithAncestors::new(self)
    }

    bfs_advance_iterator!(get_value_and_children_iter);
}

impl<'a, Node> StreamingIterator for BorrowedBFSIteratorWithAncestors<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    type Item = [Node::BorrowedValue];

    bfs_streaming_iterator_impl!(get_value_and_children_iter);
}

pub struct BorrowedBinaryBFSIterator<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    root: Option<&'a Node>,
    traversal_queue: VecDeque<BinaryChildren<&'a Node>>,
}

impl<'a, Node> BorrowedBinaryBFSIterator<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    pub(crate) fn new(root: &'a Node) -> BorrowedBinaryBFSIterator<'a, Node> {
        BorrowedBinaryBFSIterator {
            root: Some(root),
            traversal_queue: VecDeque::new(),
        }
    }

    #[doc = include_str!("../../doc_files/leaves.md")]
    pub fn leaves(self) -> BorrowedBinaryLeavesIterator<'a, Node> {
        BorrowedBinaryLeavesIterator {
            root: self.root,
            old_traversal_queue: self.traversal_queue,
            new_traversal_queue: VecDeque::new(),
        }
    }

    #[doc = include_str!("../../doc_files/attach_ancestors.md")]
    pub fn attach_ancestors(self) -> BorrowedBinaryBFSIteratorWithAncestors<'a, Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a BFS iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => BorrowedBinaryBFSIteratorWithAncestors::new(root)
        }
    }
}

impl<'a, Node> Iterator for BorrowedBinaryBFSIterator<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    type Item = Node::BorrowedValue;
    bfs_next!(get_value_and_children_iter);
}

pub struct BorrowedBinaryBFSIteratorWithAncestors<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    pub(crate) is_root: bool,
    pub(crate) item_stack: Vec<Node::BorrowedValue>,
    pub(crate) tree_cache: TreeNodeVecDeque<Node::BorrowedValue>,
    pub(crate) traversal_stack: Vec<TreeNodeVecDeque<Node::BorrowedValue>>,
    pub(crate) iterator_queue: VecDeque<BinaryChildren<&'a Node>>,
}

impl<'a, Node> BorrowedBinaryBFSIteratorWithAncestors<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    fn new(root: &'a Node) -> BorrowedBinaryBFSIteratorWithAncestors<'a, Node> {
        let (value, children) = root.get_value_and_children_iter();
        let tree_cache = TreeNodeVecDeque {
            value: None,
            children: None,
        };
        let mut iterator_queue = VecDeque::new();
        let mut item_stack = Vec::new();

        item_stack.push(value);
        iterator_queue.push_back(children);

        BorrowedBinaryBFSIteratorWithAncestors {
            is_root: true,
            item_stack,
            iterator_queue,
            traversal_stack: Vec::new(),
            tree_cache,
        }
    }

    #[doc = include_str!("../../doc_files/ancestors_leaves.md")]
    pub fn leaves(self) -> BorrowedBinaryBFSLeavesIteratorWithAncestors<'a, Node> {
        BorrowedBinaryBFSLeavesIteratorWithAncestors::new(self)
    }

    bfs_advance_iterator!(get_value_and_children_iter);
}

impl<'a, Node> StreamingIterator for BorrowedBinaryBFSIteratorWithAncestors<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    type Item = [Node::BorrowedValue];

    bfs_streaming_iterator_impl!(get_value_and_children_iter);
}
