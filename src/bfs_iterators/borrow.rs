use alloc::{collections::VecDeque, vec::Vec};
use core::array::IntoIter;
use streaming_iterator::StreamingIterator;

use crate::{
    leaves_iterators::{
        ancestors_breadth_first::borrow::{
            BorrowedBFSLeavesIteratorWithAncestors, BorrowedBinaryBFSLeavesIteratorWithAncestors,
        },
        breadth_first::borrow::{BorrowedBinaryLeavesIterator, BorrowedLeavesIterator},
    },
    prelude::{BinaryChildren, BorrowedBinaryTreeNode, BorrowedTreeNode, TreeContext},
};

use super::{
    bfs_ancestors_advance_iterator, bfs_ancestors_streaming_iterator_impl,
    bfs_context_advance_iterator, bfs_context_binary_streaming_iterator_impl,
    bfs_context_streaming_iterator_impl, bfs_next, TreeNodeVecDeque,
};

crate::collection_iterators::borrowed_collection_iterator_impl!(
    BorrowedBFSCollectionIterator,
    BorrowedBFSIterator,
    BorrowedTreeNode
);

impl<'a, IntoIter, Node> BorrowedBFSCollectionIterator<'a, IntoIter, Node>
where
    IntoIter: IntoIterator<Item = &'a Node>,
    Node: BorrowedTreeNode<'a>,
{
    #[doc = include_str!("../../doc_files/collection_attach_context.md")]
    pub fn attach_context(self) -> BorrowedBFSCollectionIteratorWithContext<'a, IntoIter, Node> {
        BorrowedBFSCollectionIteratorWithContext::new(self)
    }
}

crate::collection_iterators::borrowed_collection_context_iterator_impl!(
    BorrowedBFSCollectionIteratorWithContext,
    BorrowedBFSIteratorWithContext,
    BorrowedBFSCollectionIterator
);

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

    #[doc = include_str!("../../doc_files/attach_context.md")]
    pub fn attach_context(self) -> BorrowedBFSIteratorWithContext<'a, Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a BFS iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => BorrowedBFSIteratorWithContext::new(root, Vec::new())
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

pub struct BorrowedBFSIteratorWithContext<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    pub(crate) is_root: bool,
    pub(crate) tree_cache: TreeNodeVecDeque<Node::BorrowedValue>,
    pub(crate) traversal_stack: Vec<TreeNodeVecDeque<Node::BorrowedValue>>,
    pub(crate) iterator_queue: VecDeque<<Node::BorrowedChildren as IntoIterator>::IntoIter>,
    pub(crate) current_context: TreeContext<Node::BorrowedValue, Node::BorrowedChildren>,
    pub(crate) path_counter: usize,
}

impl<'a, Node> BorrowedBFSIteratorWithContext<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    fn new(root: &'a Node, path: Vec<usize>) -> Self {
        let (value, children) = root.get_value_and_children_iter();
        let tree_cache = TreeNodeVecDeque::default();

        let iterator_queue = VecDeque::new();
        let mut current_context = TreeContext {
            path,
            ancestors: Vec::new(),
            children: Some(children),
        };
        current_context.ancestors.push(value);

        BorrowedBFSIteratorWithContext {
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

impl<'a, Node> StreamingIterator for BorrowedBFSIteratorWithContext<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    type Item = TreeContext<Node::BorrowedValue, Node::BorrowedChildren>;

    bfs_context_streaming_iterator_impl!(get_value_and_children_iter);
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
        let tree_cache = TreeNodeVecDeque::default();
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
        BorrowedBFSLeavesIteratorWithAncestors {
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

    bfs_ancestors_advance_iterator!(get_value_and_children_iter);
}

impl<'a, Node> StreamingIterator for BorrowedBFSIteratorWithAncestors<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    type Item = [Node::BorrowedValue];

    bfs_ancestors_streaming_iterator_impl!(get_value_and_children_iter);
}

crate::collection_iterators::borrowed_collection_iterator_impl!(
    BorrowedBinaryBFSCollectionIterator,
    BorrowedBinaryBFSIterator,
    BorrowedBinaryTreeNode
);

impl<'a, IntoIter, Node> BorrowedBinaryBFSCollectionIterator<'a, IntoIter, Node>
where
    IntoIter: IntoIterator<Item = &'a Node>,
    Node: BorrowedBinaryTreeNode<'a>,
{
    #[doc = include_str!("../../doc_files/collection_attach_context.md")]
    pub fn attach_context(
        self,
    ) -> BorrowedBinaryBFSCollectionIteratorWithContext<'a, IntoIter, Node> {
        BorrowedBinaryBFSCollectionIteratorWithContext::new(self)
    }
}

crate::collection_iterators::borrowed_binary_collection_context_iterator_impl!(
    BorrowedBinaryBFSCollectionIteratorWithContext,
    BorrowedBinaryBFSIteratorWithContext,
    BorrowedBinaryBFSCollectionIterator
);

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

    #[doc = include_str!("../../doc_files/attach_context.md")]
    pub fn attach_context(self) -> BorrowedBinaryBFSIteratorWithContext<'a, Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a BFS iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => BorrowedBinaryBFSIteratorWithContext::new(root, Vec::new())
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
        let tree_cache = TreeNodeVecDeque::default();
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

    bfs_ancestors_advance_iterator!(get_value_and_children_iter);
}

impl<'a, Node> StreamingIterator for BorrowedBinaryBFSIteratorWithAncestors<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    type Item = [Node::BorrowedValue];

    bfs_ancestors_streaming_iterator_impl!(get_value_and_children_iter);
}

pub struct BorrowedBinaryBFSIteratorWithContext<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    pub(crate) is_root: bool,
    pub(crate) tree_cache: TreeNodeVecDeque<Node::BorrowedValue>,
    pub(crate) traversal_stack: Vec<TreeNodeVecDeque<Node::BorrowedValue>>,
    pub(crate) iterator_queue: VecDeque<IntoIter<Option<&'a Node>, 2>>,
    pub(crate) current_context: TreeContext<Node::BorrowedValue, [Option<&'a Node>; 2]>,
    pub(crate) path_counter: usize,
}

impl<'a, Node> BorrowedBinaryBFSIteratorWithContext<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    fn new(root: &'a Node, path: Vec<usize>) -> Self {
        let (value, children) = root.get_value_and_children_binary_iter();
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

impl<'a, Node> StreamingIterator for BorrowedBinaryBFSIteratorWithContext<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    type Item = TreeContext<Node::BorrowedValue, [Option<&'a Node>; 2]>;
    bfs_context_binary_streaming_iterator_impl!(get_value_and_children_binary_iter);
}
