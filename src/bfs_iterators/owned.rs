use core::{array::IntoIter, mem::MaybeUninit};

use alloc::{collections::VecDeque, vec::Vec};
use streaming_iterator::{StreamingIterator, StreamingIteratorMut};

use super::{
    bfs_ancestors_advance_iterator, bfs_ancestors_streaming_iterator_impl,
    bfs_context_advance_iterator, bfs_context_binary_streaming_iterator_impl,
    bfs_context_streaming_iterator_impl, bfs_next, get_mut_ancestors, get_mut_context,
    TreeNodeVecDeque,
};
use crate::{
    leaves_iterators::{
        ancestors_breadth_first::owned::{
            OwnedBFSLeavesIteratorWithAncestors, OwnedBinaryBFSLeavesIteratorWithAncestors,
        },
        breadth_first::owned::{OwnedBinaryLeavesIterator, OwnedLeavesIterator},
    },
    prelude::{BinaryChildren, OwnedBinaryTreeNode, OwnedTreeNode, TreeContext},
    tree_context::BinaryTreeContext,
};

pub struct OwnedBFSIterator<Node>
where
    Node: OwnedTreeNode,
{
    root: Option<Node>,
    traversal_queue: VecDeque<<Node::OwnedChildren as IntoIterator>::IntoIter>,
}

impl<Node> OwnedBFSIterator<Node>
where
    Node: OwnedTreeNode,
{
    pub(crate) fn new(root: Node) -> OwnedBFSIterator<Node> {
        OwnedBFSIterator {
            root: Some(root),
            traversal_queue: VecDeque::new(),
        }
    }

    #[doc = include_str!("../../doc_files/leaves.md")]
    pub fn leaves(
        self,
    ) -> OwnedLeavesIterator<Node, <Node::OwnedChildren as IntoIterator>::IntoIter> {
        OwnedLeavesIterator {
            root: self.root,
            old_traversal_queue: self.traversal_queue,
            new_traversal_queue: VecDeque::new(),
        }
    }

    #[doc = include_str!("../../doc_files/attach_context.md")]
    pub fn attach_context(self) -> OwnedBFSIteratorWithContext<Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a BFS iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => OwnedBFSIteratorWithContext::new(root)
        }
    }

    #[doc = include_str!("../../doc_files/attach_ancestors.md")]
    pub fn attach_ancestors(self) -> OwnedBFSIteratorWithAncestors<Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a BFS iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => OwnedBFSIteratorWithAncestors::new(root)
        }
    }
}

impl<Node> Iterator for OwnedBFSIterator<Node>
where
    Node: OwnedTreeNode,
{
    type Item = Node::OwnedValue;
    bfs_next!(get_value_and_children);
}

pub struct OwnedBFSIteratorWithContext<Node>
where
    Node: OwnedTreeNode,
{
    pub(crate) is_root: bool,
    pub(crate) tree_cache: TreeNodeVecDeque<Node::OwnedValue>,
    pub(crate) traversal_stack: Vec<TreeNodeVecDeque<Node::OwnedValue>>,
    pub(crate) iterator_queue: VecDeque<<Node::OwnedChildren as IntoIterator>::IntoIter>,
    pub(crate) current_context: TreeContext<Node>,
    pub(crate) path_counter: usize,
}

impl<'a, Node> OwnedBFSIteratorWithContext<Node>
where
    Node: OwnedTreeNode,
{
    fn new(root: Node) -> OwnedBFSIteratorWithContext<Node> {
        let (value, children) = root.get_value_and_children();
        let tree_cache = TreeNodeVecDeque::default();

        let iterator_queue = VecDeque::new();
        let mut current_context = TreeContext::new();
        current_context.ancestors.push(value);
        current_context.children = MaybeUninit::new(children);

        OwnedBFSIteratorWithContext {
            is_root: true,
            iterator_queue,
            traversal_stack: Vec::new(),
            tree_cache,
            current_context,
            path_counter: 0,
        }
    }

    #[doc = include_str!("../../doc_files/ancestors_leaves.md")]
    pub fn leaves(mut self) -> OwnedBFSLeavesIteratorWithAncestors<Node> {
        if !self.is_done() {
            self.iterator_queue
                .push_back(unsafe { self.current_context.children.assume_init() }.into_iter());
        }

        OwnedBFSLeavesIteratorWithAncestors {
            is_root: self.is_root,
            item_stack: self.current_context.ancestors,
            iterator_queue: self
                .iterator_queue
                .into_iter()
                .map(|val| val.peekable())
                .collect(),
            traversal_stack: self.traversal_stack,
            tree_cache: self.tree_cache,
        }
    }

    bfs_context_advance_iterator!();
}

impl<'a, Node> StreamingIterator for OwnedBFSIteratorWithContext<Node>
where
    Node: OwnedTreeNode,
{
    type Item = TreeContext<Node>;

    bfs_context_streaming_iterator_impl!(get_value_and_children);
}

impl<'a, Node> StreamingIteratorMut for OwnedBFSIteratorWithContext<Node>
where
    Node: OwnedTreeNode,
{
    get_mut_context!();
}

pub struct OwnedBFSIteratorWithAncestors<Node>
where
    Node: OwnedTreeNode,
{
    pub(crate) is_root: bool,
    pub(crate) item_stack: Vec<Node::OwnedValue>,
    pub(crate) tree_cache: TreeNodeVecDeque<Node::OwnedValue>,
    pub(crate) traversal_stack: Vec<TreeNodeVecDeque<Node::OwnedValue>>,
    pub(crate) iterator_queue: VecDeque<<Node::OwnedChildren as IntoIterator>::IntoIter>,
}

impl<'a, Node> OwnedBFSIteratorWithAncestors<Node>
where
    Node: OwnedTreeNode,
{
    fn new(root: Node) -> OwnedBFSIteratorWithAncestors<Node> {
        let (value, children) = root.get_value_and_children();
        let tree_cache = TreeNodeVecDeque::default();
        let mut iterator_queue = VecDeque::new();
        let mut item_stack = Vec::new();

        item_stack.push(value);
        iterator_queue.push_back(children.into_iter());

        OwnedBFSIteratorWithAncestors {
            is_root: true,
            item_stack,
            iterator_queue,
            traversal_stack: Vec::new(),
            tree_cache,
        }
    }

    #[doc = include_str!("../../doc_files/ancestors_leaves.md")]
    pub fn leaves(self) -> OwnedBFSLeavesIteratorWithAncestors<Node> {
        OwnedBFSLeavesIteratorWithAncestors {
            is_root: self.is_root,
            item_stack: self.item_stack,
            iterator_queue: self
                .iterator_queue
                .into_iter()
                .map(|val| val.peekable())
                .collect(),
            traversal_stack: self.traversal_stack,
            tree_cache: self.tree_cache,
        }
    }

    bfs_ancestors_advance_iterator!(get_value_and_children);
}

impl<'a, Node> StreamingIterator for OwnedBFSIteratorWithAncestors<Node>
where
    Node: OwnedTreeNode,
{
    type Item = [Node::OwnedValue];

    bfs_ancestors_streaming_iterator_impl!(get_value_and_children);
}

impl<'a, Node> StreamingIteratorMut for OwnedBFSIteratorWithAncestors<Node>
where
    Node: OwnedTreeNode,
{
    get_mut_ancestors!();
}

pub struct OwnedBinaryBFSIterator<Node>
where
    Node: OwnedBinaryTreeNode,
{
    root: Option<Node>,
    traversal_queue: VecDeque<BinaryChildren<Node>>,
}

impl<Node> OwnedBinaryBFSIterator<Node>
where
    Node: OwnedBinaryTreeNode,
{
    pub(crate) fn new(root: Node) -> OwnedBinaryBFSIterator<Node> {
        OwnedBinaryBFSIterator {
            root: Some(root),
            traversal_queue: VecDeque::new(),
        }
    }

    #[doc = include_str!("../../doc_files/leaves.md")]
    pub fn leaves(self) -> OwnedBinaryLeavesIterator<Node, BinaryChildren<Node>> {
        OwnedBinaryLeavesIterator {
            root: self.root,
            old_traversal_queue: self.traversal_queue,
            new_traversal_queue: VecDeque::new(),
        }
    }

    #[doc = include_str!("../../doc_files/attach_context.md")]
    pub fn attach_context(self) -> OwnedBinaryBFSIteratorWithContext<Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a BFS iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => OwnedBinaryBFSIteratorWithContext::new(root)
        }
    }

    #[doc = include_str!("../../doc_files/attach_ancestors.md")]
    pub fn attach_ancestors(self) -> OwnedBinaryBFSIteratorWithAncestors<Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a BFS iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => OwnedBinaryBFSIteratorWithAncestors::new(root)
        }
    }
}

impl<Node> Iterator for OwnedBinaryBFSIterator<Node>
where
    Node: OwnedBinaryTreeNode,
{
    type Item = Node::OwnedValue;
    bfs_next!(get_value_and_children);
}

pub struct OwnedBinaryBFSIteratorWithAncestors<Node>
where
    Node: OwnedBinaryTreeNode,
{
    pub(crate) is_root: bool,
    pub(crate) item_stack: Vec<Node::OwnedValue>,
    pub(crate) tree_cache: TreeNodeVecDeque<Node::OwnedValue>,
    pub(crate) traversal_stack: Vec<TreeNodeVecDeque<Node::OwnedValue>>,
    pub(crate) iterator_queue: VecDeque<BinaryChildren<Node>>,
}

impl<'a, Node> OwnedBinaryBFSIteratorWithAncestors<Node>
where
    Node: OwnedBinaryTreeNode,
{
    fn new(root: Node) -> OwnedBinaryBFSIteratorWithAncestors<Node> {
        let (value, children) = root.get_value_and_children();
        let tree_cache = TreeNodeVecDeque::default();
        let mut iterator_queue = VecDeque::new();
        let mut item_stack = Vec::new();

        item_stack.push(value);
        iterator_queue.push_back(children);

        OwnedBinaryBFSIteratorWithAncestors {
            is_root: true,
            item_stack,
            iterator_queue,
            traversal_stack: Vec::new(),
            tree_cache,
        }
    }

    #[doc = include_str!("../../doc_files/ancestors_leaves.md")]
    pub fn leaves(self) -> OwnedBinaryBFSLeavesIteratorWithAncestors<Node> {
        OwnedBinaryBFSLeavesIteratorWithAncestors::new(self)
    }

    bfs_ancestors_advance_iterator!(get_value_and_children);
}

impl<'a, Node> StreamingIterator for OwnedBinaryBFSIteratorWithAncestors<Node>
where
    Node: OwnedBinaryTreeNode,
{
    type Item = [Node::OwnedValue];

    bfs_ancestors_streaming_iterator_impl!(get_value_and_children);
}

impl<'a, Node> StreamingIteratorMut for OwnedBinaryBFSIteratorWithAncestors<Node>
where
    Node: OwnedBinaryTreeNode,
{
    get_mut_ancestors!();
}

pub struct OwnedBinaryBFSIteratorWithContext<Node>
where
    Node: OwnedBinaryTreeNode,
{
    pub(crate) is_root: bool,
    pub(crate) tree_cache: TreeNodeVecDeque<Node::OwnedValue>,
    pub(crate) traversal_stack: Vec<TreeNodeVecDeque<Node::OwnedValue>>,
    pub(crate) iterator_queue: VecDeque<IntoIter<Option<Node>, 2>>,
    pub(crate) current_context: BinaryTreeContext<Node>,
    pub(crate) path_counter: usize,
}

impl<'a, Node> OwnedBinaryBFSIteratorWithContext<Node>
where
    Node: OwnedBinaryTreeNode,
{
    fn new(root: Node) -> Self {
        let (value, children) = root.get_value_and_children_binary();
        let tree_cache = TreeNodeVecDeque::default();

        let iterator_queue = VecDeque::new();
        let mut current_context = BinaryTreeContext::new();
        current_context.ancestors.push(value);
        current_context.children = MaybeUninit::new(children);

        Self {
            is_root: true,
            current_context,
            iterator_queue,
            traversal_stack: Vec::new(),
            tree_cache,
            path_counter: 0,
        }
    }

    #[doc = include_str!("../../doc_files/ancestors_leaves.md")]
    pub fn leaves(self) -> OwnedBinaryBFSLeavesIteratorWithAncestors<Node> {
        todo!();
    }

    bfs_context_advance_iterator!();
}

impl<'a, Node> StreamingIterator for OwnedBinaryBFSIteratorWithContext<Node>
where
    Node: OwnedBinaryTreeNode,
{
    type Item = BinaryTreeContext<Node>;
    bfs_context_binary_streaming_iterator_impl!(get_value_and_children_binary);
}

impl<'a, Node> StreamingIteratorMut for OwnedBinaryBFSIteratorWithContext<Node>
where
    Node: OwnedBinaryTreeNode,
{
    get_mut_context!();
}
