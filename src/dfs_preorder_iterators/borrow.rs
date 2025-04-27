use core::array::IntoIter;

use crate::{
    leaves_iterators::{
        ancestors_depth_first::borrow::{
            BorrowedBinaryDFSLeavesPostorderIteratorWithAncestors,
            BorrowedDFSLeavesPostorderIteratorWithAncestors,
        },
        depth_first::borrow::{BorrowedBinaryLeavesIterator, BorrowedLeavesIterator},
    },
    prelude::{
        BinaryChildren, BinaryTreeIterator, BorrowedBinaryTreeNode, BorrowedTreeNode, TreeContext,
        TreeIterator, TreeIteratorBase,
    },
};
use alloc::vec::Vec;
use streaming_iterator::StreamingIterator;

use super::{
    dfs_preorder_binary_next_with_path_tracking, dfs_preorder_next,
    dfs_preorder_next_with_path_tracking, preorder_ancestors_streaming_iterator_impl,
    preorder_binary_context_streaming_iterator_impl, preorder_context_streaming_iterator_impl,
};

crate::collection_iterators::borrowed_collection_iterator_impl!(
    BorrowedDFSPreorderCollectionIterator,
    BorrowedDFSPreorderIterator,
    BorrowedTreeNode
);

impl<'a, IntoIter, Node> BorrowedDFSPreorderCollectionIterator<'a, IntoIter, Node>
where
    IntoIter: IntoIterator<Item = &'a Node>,
    Node: BorrowedTreeNode<'a>,
{
    #[doc = include_str!("../../doc_files/collection_attach_context.md")]
    pub fn attach_context(
        self,
    ) -> BorrowedDFSPreorderCollectionIteratorWithContext<'a, IntoIter, Node> {
        BorrowedDFSPreorderCollectionIteratorWithContext::new(self)
    }
}

crate::collection_iterators::borrowed_collection_context_iterator_impl!(
    BorrowedDFSPreorderCollectionIteratorWithContext,
    BorrowedDFSPreorderIteratorWithContext,
    BorrowedDFSPreorderCollectionIterator
);

impl<'a, IntoIter, Node> BorrowedBinaryDFSPreorderCollectionIterator<'a, IntoIter, Node>
where
    IntoIter: IntoIterator<Item = &'a Node>,
    Node: BorrowedBinaryTreeNode<'a>,
{
    #[doc = include_str!("../../doc_files/collection_attach_context.md")]
    pub fn attach_context(
        self,
    ) -> BorrowedBinaryDFSPreorderCollectionIteratorWithContext<'a, IntoIter, Node> {
        BorrowedBinaryDFSPreorderCollectionIteratorWithContext::new(self)
    }
}

crate::collection_iterators::borrowed_binary_collection_context_iterator_impl!(
    BorrowedBinaryDFSPreorderCollectionIteratorWithContext,
    BorrowedBinaryDFSPreorderIteratorWithContext,
    BorrowedBinaryDFSPreorderCollectionIterator
);

pub struct BorrowedDFSPreorderIterator<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    root: Option<&'a Node>,
    traversal_stack: Vec<<Node::BorrowedChildren as IntoIterator>::IntoIter>,
}

impl<'a, Node> BorrowedDFSPreorderIterator<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    pub(crate) fn new(root: &'a Node) -> Self {
        Self {
            root: Some(root),
            traversal_stack: Vec::new(),
        }
    }

    #[doc = include_str!("../../doc_files/leaves.md")]
    pub fn leaves(
        self,
    ) -> BorrowedLeavesIterator<'a, Node, <Node::BorrowedChildren as IntoIterator>::IntoIter> {
        BorrowedLeavesIterator {
            root: self.root,
            traversal_stack_bottom: self.traversal_stack,
            traversal_stack_top: Vec::new(),
            item_stack: Vec::new(),
        }
    }

    #[doc = include_str!("../../doc_files/attach_context.md")]
    pub fn attach_context(self) -> BorrowedDFSPreorderIteratorWithContext<'a, Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS preorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                BorrowedDFSPreorderIteratorWithContext::new(root, Vec::new())
            }
        }
    }

    #[doc = include_str!("../../doc_files/attach_ancestors.md")]
    pub fn attach_ancestors(self) -> BorrowedDFSPreorderIteratorWithAncestors<'a, Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS preorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                BorrowedDFSPreorderIteratorWithAncestors::new(root)
            }
        }
    }
}

impl<'a, Node> Iterator for BorrowedDFSPreorderIterator<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    type Item = Node::BorrowedValue;
    dfs_preorder_next!(get_value_and_children_iter);
}

pub(crate) struct BorrowedDFSPreorderIteratorWithPathTracking<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    root: Option<&'a Node>,
    path: Vec<usize>,
    on_deck_into_iterator: Option<Node::BorrowedChildren>,
    traversal_stack: Vec<<Node::BorrowedChildren as IntoIterator>::IntoIter>,
}

impl<'a, Node> BorrowedDFSPreorderIteratorWithPathTracking<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    pub(crate) fn new(root: &'a Node) -> Self {
        Self {
            root: Some(root),
            path: Vec::new(),
            on_deck_into_iterator: None,
            traversal_stack: Vec::new(),
        }
    }
}

impl<'a, Node> Iterator for BorrowedDFSPreorderIteratorWithPathTracking<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    type Item = Node::BorrowedValue;
    dfs_preorder_next_with_path_tracking!(get_value_and_children_iter);
}

impl<'a, Node> crate::Sealed for BorrowedDFSPreorderIteratorWithPathTracking<'a, Node> where
    Node: BorrowedTreeNode<'a>
{
}

impl<'a, Node> TreeIteratorBase<Node::BorrowedValue, Node::BorrowedChildren>
    for BorrowedDFSPreorderIteratorWithPathTracking<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    fn current_path(&self) -> &[usize] {
        &self.path
    }

    fn prune_current_subtree(&mut self) {
        self.on_deck_into_iterator.take();
    }
}

impl<'a, Node> TreeIterator<Node::BorrowedValue, Node::BorrowedChildren>
    for BorrowedDFSPreorderIteratorWithPathTracking<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
}

pub struct BorrowedDFSPreorderIteratorWithContext<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    root: Option<&'a Node>,
    traversal_stack: Vec<<Node::BorrowedChildren as IntoIterator>::IntoIter>,
    current_context: TreeContext<Node::BorrowedValue, Node::BorrowedChildren>,
}

impl<'a, Node> BorrowedDFSPreorderIteratorWithContext<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    pub(crate) fn new(root: &'a Node, path: Vec<usize>) -> Self {
        Self {
            root: Some(root),
            traversal_stack: Vec::new(),
            current_context: TreeContext {
                path,
                ancestors: Vec::new(),
                children: None,
            },
        }
    }
}

impl<'a, Node> StreamingIterator for BorrowedDFSPreorderIteratorWithContext<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    type Item = TreeContext<Node::BorrowedValue, Node::BorrowedChildren>;
    preorder_context_streaming_iterator_impl!(get_value_and_children_iter);
}

pub struct BorrowedDFSPreorderIteratorWithAncestors<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    root: Option<&'a Node>,
    traversal_stack: Vec<<Node::BorrowedChildren as IntoIterator>::IntoIter>,
    item_stack: Vec<Node::BorrowedValue>,
}

impl<'a, Node> BorrowedDFSPreorderIteratorWithAncestors<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    pub(crate) fn new(root: &'a Node) -> Self {
        Self {
            root: Some(root),
            traversal_stack: Vec::new(),
            item_stack: Vec::new(),
        }
    }

    #[doc = include_str!("../../doc_files/ancestors_leaves.md")]
    pub fn leaves(
        self,
    ) -> BorrowedDFSLeavesPostorderIteratorWithAncestors<
        'a,
        Node,
        <Node::BorrowedChildren as IntoIterator>::IntoIter,
    > {
        BorrowedDFSLeavesPostorderIteratorWithAncestors {
            root: self.root,
            item_stack: self.item_stack,
            old_traversal_stack: self.traversal_stack,
            new_traversal_stack: Vec::new(),
        }
    }
}

impl<'a, Node> StreamingIterator for BorrowedDFSPreorderIteratorWithAncestors<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    type Item = [Node::BorrowedValue];

    preorder_ancestors_streaming_iterator_impl!(get_value_and_children_iter);
}

crate::collection_iterators::borrowed_collection_iterator_impl!(
    BorrowedBinaryDFSPreorderCollectionIterator,
    BorrowedBinaryDFSPreorderIterator,
    BorrowedBinaryTreeNode
);

pub struct BorrowedBinaryDFSPreorderIterator<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    root: Option<&'a Node>,
    traversal_stack: Vec<BinaryChildren<&'a Node>>,
}

impl<'a, Node> BorrowedBinaryDFSPreorderIterator<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    pub(crate) fn new(root: &'a Node) -> Self {
        Self {
            root: Some(root),
            traversal_stack: Vec::new(),
        }
    }

    #[doc = include_str!("../../doc_files/leaves.md")]
    pub fn leaves(self) -> BorrowedBinaryLeavesIterator<'a, Node, BinaryChildren<&'a Node>> {
        BorrowedBinaryLeavesIterator {
            root: self.root,
            traversal_stack_bottom: self.traversal_stack,
            traversal_stack_top: Vec::new(),
            item_stack: Vec::new(),
        }
    }

    #[doc = include_str!("../../doc_files/attach_context.md")]
    pub fn attach_context(self) -> BorrowedBinaryDFSPreorderIteratorWithContext<'a, Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS preorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                BorrowedBinaryDFSPreorderIteratorWithContext::new(root, Vec::new())
            }
        }
    }

    #[doc = include_str!("../../doc_files/attach_ancestors.md")]
    pub fn attach_ancestors(self) -> BorrowedBinaryDFSPreorderIteratorWithAncestors<'a, Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS preorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                BorrowedBinaryDFSPreorderIteratorWithAncestors::new(root)
            }
        }
    }
}

impl<'a, Node> Iterator for BorrowedBinaryDFSPreorderIterator<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    type Item = Node::BorrowedValue;
    dfs_preorder_next!(get_value_and_children_iter);
}

pub(crate) struct BorrowedBinaryDFSPreorderIteratorWithPathTracking<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    root: Option<&'a Node>,
    traversal_stack: Vec<IntoIter<Option<&'a Node>, 2>>,
    path: Vec<usize>,
    on_deck_into_iterator: Option<[Option<&'a Node>; 2]>,
}

impl<'a, Node> BorrowedBinaryDFSPreorderIteratorWithPathTracking<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    pub(crate) fn new(root: &'a Node) -> Self {
        Self {
            root: Some(root),
            traversal_stack: Vec::new(),
            path: Vec::new(),
            on_deck_into_iterator: None,
        }
    }
}

impl<'a, Node> Iterator for BorrowedBinaryDFSPreorderIteratorWithPathTracking<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    type Item = Node::BorrowedValue;
    dfs_preorder_binary_next_with_path_tracking!(get_value_and_children_binary_iter);
}

impl<'a, Node> crate::Sealed for BorrowedBinaryDFSPreorderIteratorWithPathTracking<'a, Node> where
    Node: BorrowedBinaryTreeNode<'a>
{
}

impl<'a, Node> TreeIteratorBase<Node::BorrowedValue, [Option<&'a Node>; 2]>
    for BorrowedBinaryDFSPreorderIteratorWithPathTracking<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    fn current_path(&self) -> &[usize] {
        &self.path
    }

    fn prune_current_subtree(&mut self) {
        self.on_deck_into_iterator.take();
    }
}

impl<'a, Node> BinaryTreeIterator<Node::BorrowedValue, [Option<&'a Node>; 2]>
    for BorrowedBinaryDFSPreorderIteratorWithPathTracking<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
}

pub struct BorrowedBinaryDFSPreorderIteratorWithAncestors<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    root: Option<&'a Node>,
    traversal_stack: Vec<BinaryChildren<&'a Node>>,
    item_stack: Vec<Node::BorrowedValue>,
}

impl<'a, Node> BorrowedBinaryDFSPreorderIteratorWithAncestors<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    pub(crate) fn new(root: &'a Node) -> Self {
        Self {
            root: Some(root),
            traversal_stack: Vec::new(),
            item_stack: Vec::new(),
        }
    }

    #[doc = include_str!("../../doc_files/ancestors_leaves.md")]
    pub fn leaves(
        self,
    ) -> BorrowedBinaryDFSLeavesPostorderIteratorWithAncestors<'a, Node, BinaryChildren<&'a Node>>
    {
        BorrowedBinaryDFSLeavesPostorderIteratorWithAncestors {
            root: self.root,
            item_stack: self.item_stack,
            old_traversal_stack: self.traversal_stack,
            new_traversal_stack: Vec::new(),
        }
    }
}

impl<'a, Node> StreamingIterator for BorrowedBinaryDFSPreorderIteratorWithAncestors<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    type Item = [Node::BorrowedValue];
    preorder_ancestors_streaming_iterator_impl!(get_value_and_children_iter);
}

pub struct BorrowedBinaryDFSPreorderIteratorWithContext<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    root: Option<&'a Node>,
    traversal_stack: Vec<IntoIter<Option<&'a Node>, 2>>,
    current_context: TreeContext<Node::BorrowedValue, [Option<&'a Node>; 2]>,
}

impl<'a, Node> BorrowedBinaryDFSPreorderIteratorWithContext<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    pub(crate) fn new(root: &'a Node, path: Vec<usize>) -> Self {
        Self {
            root: Some(root),
            traversal_stack: Vec::new(),
            current_context: TreeContext {
                path,
                ancestors: Vec::new(),
                children: None,
            },
        }
    }
}

impl<'a, Node> StreamingIterator for BorrowedBinaryDFSPreorderIteratorWithContext<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    type Item = TreeContext<Node::BorrowedValue, [Option<&'a Node>; 2]>;
    preorder_binary_context_streaming_iterator_impl!(get_value_and_children_binary_iter);
}
