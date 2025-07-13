use core::array::IntoIter;

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
        ancestors_breadth_first::mut_borrow::{
            MutBorrowedBFSLeavesIteratorWithAncestors,
            MutBorrowedBinaryBFSLeavesIteratorWithAncestors,
        },
        breadth_first::mut_borrow::{MutBorrowedBinaryLeavesIterator, MutBorrowedLeavesIterator},
    },
    prelude::{BinaryChildren, MutBorrowedBinaryTreeNode, MutBorrowedTreeNode, TreeContext},
};

crate::collection_iterators::mut_borrowed_collection_iterator_impl!(
    MutBorrowedBFSCollectionIterator,
    MutBorrowedBFSIterator,
    MutBorrowedTreeNode
);

impl<'a, IntoIter, Node> MutBorrowedBFSCollectionIterator<'a, IntoIter, Node>
where
    IntoIter: IntoIterator<Item = &'a mut Node>,
    Node: MutBorrowedTreeNode<'a>,
{
    #[doc = include_str!("../../doc_files/collection_attach_context.md")]
    pub fn attach_context(self) -> MutBorrowedBFSCollectionIteratorWithContext<'a, IntoIter, Node> {
        MutBorrowedBFSCollectionIteratorWithContext::new(self)
    }
}

crate::collection_iterators::mut_borrowed_collection_context_iterator_impl!(
    MutBorrowedBFSCollectionIteratorWithContext,
    MutBorrowedBFSIteratorWithContext,
    MutBorrowedBFSCollectionIterator
);

pub struct MutBorrowedBFSIterator<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    root: Option<&'a mut Node>,
    traversal_queue: VecDeque<<Node::MutBorrowedChildren as IntoIterator>::IntoIter>,
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

    #[doc = include_str!("../../doc_files/leaves.md")]
    pub fn leaves(self) -> MutBorrowedLeavesIterator<'a, Node> {
        MutBorrowedLeavesIterator {
            root: self.root,
            old_traversal_queue: self.traversal_queue,
            new_traversal_queue: VecDeque::new(),
        }
    }

    #[doc = include_str!("../../doc_files/attach_context.md")]
    pub fn attach_context(self) -> MutBorrowedBFSIteratorWithContext<'a, Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a BFS iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => MutBorrowedBFSIteratorWithContext::new(root, Vec::new())
        }
    }

    #[doc = include_str!("../../doc_files/attach_ancestors.md")]
    pub fn attach_ancestors(self) -> MutBorrowedBFSIteratorWithAncestors<'a, Node> {
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

pub struct MutBorrowedBFSIteratorWithContext<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    pub(crate) is_root: bool,
    pub(crate) tree_cache: TreeNodeVecDeque<Node::MutBorrowedValue>,
    pub(crate) traversal_stack: Vec<TreeNodeVecDeque<Node::MutBorrowedValue>>,
    pub(crate) iterator_queue: VecDeque<<Node::MutBorrowedChildren as IntoIterator>::IntoIter>,
    pub(crate) current_context: TreeContext<Node::MutBorrowedValue, Node::MutBorrowedChildren>,
    pub(crate) path_counter: usize,
}

impl<'a, Node> MutBorrowedBFSIteratorWithContext<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    fn new(root: &'a mut Node, path: Vec<usize>) -> MutBorrowedBFSIteratorWithContext<'a, Node> {
        let (value, children) = root.get_value_and_children_iter_mut();
        let tree_cache = TreeNodeVecDeque::default();
        let iterator_queue = VecDeque::new();
        let mut current_context = TreeContext {
            path,
            ancestors: Vec::new(),
            children: Some(children),
        };
        current_context.ancestors.push(value);

        MutBorrowedBFSIteratorWithContext {
            is_root: true,
            iterator_queue,
            traversal_stack: Vec::new(),
            tree_cache,
            current_context,
            path_counter: 0,
        }
    }

    bfs_context_advance_iterator!();
}

impl<'a, Node> StreamingIterator for MutBorrowedBFSIteratorWithContext<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    type Item = TreeContext<Node::MutBorrowedValue, Node::MutBorrowedChildren>;

    bfs_context_streaming_iterator_impl!(get_value_and_children_iter_mut);
}

impl<'a, Node> StreamingIteratorMut for MutBorrowedBFSIteratorWithContext<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    get_mut_context!();
}

pub struct MutBorrowedBFSIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    pub(crate) is_root: bool,
    pub(crate) item_stack: Vec<Node::MutBorrowedValue>,
    pub(crate) tree_cache: TreeNodeVecDeque<Node::MutBorrowedValue>,
    pub(crate) traversal_stack: Vec<TreeNodeVecDeque<Node::MutBorrowedValue>>,
    pub(crate) iterator_queue: VecDeque<<Node::MutBorrowedChildren as IntoIterator>::IntoIter>,
}

impl<'a, Node> MutBorrowedBFSIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    fn new(root: &'a mut Node) -> MutBorrowedBFSIteratorWithAncestors<'a, Node> {
        let (value, children) = root.get_value_and_children_iter_mut();
        let tree_cache = TreeNodeVecDeque::default();
        let mut iterator_queue = VecDeque::new();
        let mut item_stack = Vec::new();

        item_stack.push(value);
        iterator_queue.push_back(children.into_iter());

        MutBorrowedBFSIteratorWithAncestors {
            is_root: true,
            item_stack,
            iterator_queue,
            traversal_stack: Vec::new(),
            tree_cache,
        }
    }

    #[doc = include_str!("../../doc_files/ancestors_leaves.md")]
    pub fn leaves(self) -> MutBorrowedBFSLeavesIteratorWithAncestors<'a, Node> {
        MutBorrowedBFSLeavesIteratorWithAncestors {
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

    bfs_ancestors_advance_iterator!(get_value_and_children_iter_mut);
}

impl<'a, Node> StreamingIterator for MutBorrowedBFSIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    type Item = [Node::MutBorrowedValue];

    bfs_ancestors_streaming_iterator_impl!(get_value_and_children_iter_mut);
}

impl<'a, Node> StreamingIteratorMut for MutBorrowedBFSIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    get_mut_ancestors!();
}

crate::collection_iterators::mut_borrowed_collection_iterator_impl!(
    MutBorrowedBinaryBFSCollectionIterator,
    MutBorrowedBinaryBFSIterator,
    MutBorrowedBinaryTreeNode
);

impl<'a, IntoIter, Node> MutBorrowedBinaryBFSCollectionIterator<'a, IntoIter, Node>
where
    IntoIter: IntoIterator<Item = &'a mut Node>,
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    #[doc = include_str!("../../doc_files/collection_attach_context.md")]
    pub fn attach_context(
        self,
    ) -> MutBorrowedBinaryBFSCollectionIteratorWithContext<'a, IntoIter, Node> {
        MutBorrowedBinaryBFSCollectionIteratorWithContext::new(self)
    }
}

crate::collection_iterators::mut_borrowed_binary_collection_context_iterator_impl!(
    MutBorrowedBinaryBFSCollectionIteratorWithContext,
    MutBorrowedBinaryBFSIteratorWithContext,
    MutBorrowedBinaryBFSCollectionIterator
);

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

    #[doc = include_str!("../../doc_files/leaves.md")]
    pub fn leaves(self) -> MutBorrowedBinaryLeavesIterator<'a, Node, BinaryChildren<&'a mut Node>> {
        MutBorrowedBinaryLeavesIterator {
            root: self.root,
            old_traversal_queue: self.traversal_queue,
            new_traversal_queue: VecDeque::new(),
        }
    }

    #[doc = include_str!("../../doc_files/attach_context.md")]
    pub fn attach_context(self) -> MutBorrowedBinaryBFSIteratorWithContext<'a, Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a BFS iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => MutBorrowedBinaryBFSIteratorWithContext::new(root, Vec::new())
        }
    }

    #[doc = include_str!("../../doc_files/attach_ancestors.md")]
    pub fn attach_ancestors(self) -> MutBorrowedBinaryBFSIteratorWithAncestors<'a, Node> {
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
        let tree_cache = TreeNodeVecDeque::default();
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

    #[doc = include_str!("../../doc_files/ancestors_leaves.md")]
    pub fn leaves(self) -> MutBorrowedBinaryBFSLeavesIteratorWithAncestors<'a, Node> {
        MutBorrowedBinaryBFSLeavesIteratorWithAncestors::new(self)
    }

    bfs_ancestors_advance_iterator!(get_value_and_children_iter_mut);
}

impl<'a, Node> StreamingIterator for MutBorrowedBinaryBFSIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    type Item = [Node::MutBorrowedValue];

    bfs_ancestors_streaming_iterator_impl!(get_value_and_children_iter_mut);
}

impl<'a, Node> StreamingIteratorMut for MutBorrowedBinaryBFSIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    get_mut_ancestors!();
}

pub struct MutBorrowedBinaryBFSIteratorWithContext<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    pub(crate) is_root: bool,
    pub(crate) tree_cache: TreeNodeVecDeque<Node::MutBorrowedValue>,
    pub(crate) traversal_stack: Vec<TreeNodeVecDeque<Node::MutBorrowedValue>>,
    pub(crate) iterator_queue: VecDeque<IntoIter<Option<&'a mut Node>, 2>>,
    pub(crate) current_context: TreeContext<Node::MutBorrowedValue, [Option<&'a mut Node>; 2]>,
    pub(crate) path_counter: usize,
}

impl<'a, Node> MutBorrowedBinaryBFSIteratorWithContext<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    fn new(root: &'a mut Node, path: Vec<usize>) -> Self {
        let (value, children) = root.get_value_and_children_binary_iter_mut();
        let tree_cache = TreeNodeVecDeque::default();

        let iterator_queue = VecDeque::new();
        let mut current_context = TreeContext {
            path,
            ancestors: Vec::new(),
            children: Some(children),
        };
        current_context.ancestors.push(value);

        Self {
            is_root: true,
            current_context,
            iterator_queue,
            traversal_stack: Vec::new(),
            tree_cache,
            path_counter: 0,
        }
    }

    bfs_context_advance_iterator!();
}

impl<'a, Node> StreamingIterator for MutBorrowedBinaryBFSIteratorWithContext<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    type Item = TreeContext<Node::MutBorrowedValue, [Option<&'a mut Node>; 2]>;
    bfs_context_binary_streaming_iterator_impl!(get_value_and_children_binary_iter_mut);
}

impl<'a, Node> StreamingIteratorMut for MutBorrowedBinaryBFSIteratorWithContext<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    get_mut_context!();
}
