use core::{
    array::IntoIter,
    iter::{Enumerate, Fuse},
};

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
        BinaryChildren, BinaryTreeCollectionIterator, BinaryTreeIterator,
        MutBorrowedBinaryTreeNode, MutBorrowedTreeNode, TreeCollectionIterator,
        TreeCollectionIteratorBase, TreeContext, TreeIterator, TreeIteratorBase,
    },
};

use super::{
    dfs_preorder_binary_next_with_path_tracking, dfs_preorder_next,
    dfs_preorder_next_with_path_tracking, get_mut_ancestors, get_mut_context,
    preorder_ancestors_streaming_iterator_impl, preorder_binary_context_streaming_iterator_impl,
    preorder_context_streaming_iterator_impl,
};

crate::collection_iterators::mut_borrowed_collection_iterator_impl!(
    MutBorrowedDFSPreorderCollectionIterator,
    MutBorrowedDFSPreorderIterator,
    MutBorrowedTreeNode
);

impl<'a, IntoIter, Node> MutBorrowedDFSPreorderCollectionIterator<'a, IntoIter, Node>
where
    IntoIter: IntoIterator<Item = &'a mut Node>,
    Node: MutBorrowedTreeNode<'a>,
{
    #[doc = include_str!("../../doc_files/collection_attach_context.md")]
    pub fn attach_context(
        self,
    ) -> MutBorrowedDFSPreorderCollectionIteratorWithContext<'a, IntoIter, Node> {
        MutBorrowedDFSPreorderCollectionIteratorWithContext::new(self)
    }
}

crate::collection_iterators::mut_borrowed_collection_context_iterator_impl!(
    MutBorrowedDFSPreorderCollectionIteratorWithContext,
    MutBorrowedDFSPreorderIteratorWithContext,
    MutBorrowedDFSPreorderCollectionIterator
);

pub(crate) struct MutBorrowedDFSPreorderCollectionIteratorWithPathTracking<'a, Node, IntoIter>
where
    IntoIter: IntoIterator<Item = &'a mut Node>,
    Node: MutBorrowedTreeNode<'a>,
{
    tree_collection: Enumerate<Fuse<IntoIter::IntoIter>>,
    current_tree_iterator: Option<MutBorrowedDFSPreorderIteratorWithPathTracking<'a, Node>>,
}

impl<'a, Node, IntoIter>
    MutBorrowedDFSPreorderCollectionIteratorWithPathTracking<'a, Node, IntoIter>
where
    IntoIter: IntoIterator<Item = &'a mut Node>,
    Node: MutBorrowedTreeNode<'a>,
{
    pub(crate) fn new(into_iter: IntoIter) -> Self {
        Self {
            tree_collection: into_iter.into_iter().fuse().enumerate(),
            current_tree_iterator: None,
        }
    }
}

impl<'a, Node, IntoIter> Iterator
    for MutBorrowedDFSPreorderCollectionIteratorWithPathTracking<'a, Node, IntoIter>
where
    IntoIter: IntoIterator<Item = &'a mut Node>,
    Node: MutBorrowedTreeNode<'a>,
{
    type Item = Node::MutBorrowedValue;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(current_tree_iterator) = &mut self.current_tree_iterator {
                if let Some(current) = current_tree_iterator.next() {
                    return Some(current);
                }
            }

            if let Some(next_tree) = self.tree_collection.next() {
                let mut path = Vec::new();
                path.push(next_tree.0);
                self.current_tree_iterator = Some(
                    MutBorrowedDFSPreorderIteratorWithPathTracking::new(next_tree.1, path),
                );
            } else {
                return None;
            }
        }
    }
}

impl<'a, Node, IntoIter>
    TreeCollectionIteratorBase<Node::MutBorrowedValue, Node::MutBorrowedChildren>
    for MutBorrowedDFSPreorderCollectionIteratorWithPathTracking<'a, Node, IntoIter>
where
    IntoIter: IntoIterator<Item = &'a mut Node>,
    Node: MutBorrowedTreeNode<'a>,
{
    fn current_path(&self) -> &[usize] {
        self.current_tree_iterator
            .as_ref()
            .map(|iter| iter.current_path())
            .unwrap_or(&[])
    }

    fn prune_current_subtree(&mut self) {
        if let Some(inner) = self.current_tree_iterator.as_mut() {
            inner.prune_current_subtree();
        }
    }
}

impl<'a, Node, IntoIter> TreeCollectionIterator<Node::MutBorrowedValue, Node::MutBorrowedChildren>
    for MutBorrowedDFSPreorderCollectionIteratorWithPathTracking<'a, Node, IntoIter>
where
    IntoIter: IntoIterator<Item = &'a mut Node>,
    Node: MutBorrowedTreeNode<'a>,
{
}

pub struct MutBorrowedDFSPreorderIterator<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    root: Option<&'a mut Node>,
    traversal_stack: Vec<<Node::MutBorrowedChildren as IntoIterator>::IntoIter>,
}

impl<'a, Node> MutBorrowedDFSPreorderIterator<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    pub(crate) fn new(root: &'a mut Node) -> Self {
        Self {
            root: Some(root),
            traversal_stack: Vec::new(),
        }
    }

    #[doc = include_str!("../../doc_files/leaves.md")]
    pub fn leaves(
        self,
    ) -> MutBorrowedLeavesIterator<'a, Node, <Node::MutBorrowedChildren as IntoIterator>::IntoIter>
    {
        MutBorrowedLeavesIterator {
            root: self.root,
            traversal_stack_bottom: self.traversal_stack,
            traversal_stack_top: Vec::new(),
            item_stack: Vec::new(),
        }
    }

    #[doc = include_str!("../../doc_files/attach_context.md")]
    pub fn attach_context(self) -> MutBorrowedDFSPreorderIteratorWithContext<'a, Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS preorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                MutBorrowedDFSPreorderIteratorWithContext::new(root, Vec::new())
            }
        }
    }

    #[doc = include_str!("../../doc_files/attach_ancestors.md")]
    pub fn attach_ancestors(self) -> MutBorrowedDFSPreorderIteratorWithAncestors<'a, Node> {
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

pub(crate) struct MutBorrowedDFSPreorderIteratorWithPathTracking<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    root: Option<&'a mut Node>,
    path: Vec<usize>,
    on_deck_into_iterator: Option<Node::MutBorrowedChildren>,
    traversal_stack: Vec<<Node::MutBorrowedChildren as IntoIterator>::IntoIter>,
}

impl<'a, Node> MutBorrowedDFSPreorderIteratorWithPathTracking<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    pub(crate) fn new(root: &'a mut Node, path: Vec<usize>) -> Self {
        Self {
            root: Some(root),
            path,
            on_deck_into_iterator: None,
            traversal_stack: Vec::new(),
        }
    }
}

impl<'a, Node> Iterator for MutBorrowedDFSPreorderIteratorWithPathTracking<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    type Item = Node::MutBorrowedValue;
    dfs_preorder_next_with_path_tracking!(get_value_and_children_iter_mut);
}

impl<'a, Node> TreeIteratorBase<Node::MutBorrowedValue, Node::MutBorrowedChildren>
    for MutBorrowedDFSPreorderIteratorWithPathTracking<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    fn current_path(&self) -> &[usize] {
        &self.path
    }

    fn prune_current_subtree(&mut self) {
        self.on_deck_into_iterator.take();
    }
}

impl<'a, Node> TreeIterator<Node::MutBorrowedValue, Node::MutBorrowedChildren>
    for MutBorrowedDFSPreorderIteratorWithPathTracking<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
}

pub struct MutBorrowedDFSPreorderIteratorWithContext<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    root: Option<&'a mut Node>,
    traversal_stack: Vec<<Node::MutBorrowedChildren as IntoIterator>::IntoIter>,
    current_context: TreeContext<Node::MutBorrowedValue, Node::MutBorrowedChildren>,
}

impl<'a, Node> MutBorrowedDFSPreorderIteratorWithContext<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    pub(crate) fn new(root: &'a mut Node, path: Vec<usize>) -> Self {
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

impl<'a, Node> StreamingIterator for MutBorrowedDFSPreorderIteratorWithContext<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    type Item = TreeContext<Node::MutBorrowedValue, Node::MutBorrowedChildren>;
    preorder_context_streaming_iterator_impl!(get_value_and_children_iter_mut);
}

impl<'a, Node> StreamingIteratorMut for MutBorrowedDFSPreorderIteratorWithContext<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    get_mut_context!();
}

pub struct MutBorrowedDFSPreorderIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    root: Option<&'a mut Node>,
    traversal_stack: Vec<<Node::MutBorrowedChildren as IntoIterator>::IntoIter>,
    item_stack: Vec<Node::MutBorrowedValue>,
}

impl<'a, Node> MutBorrowedDFSPreorderIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    pub(crate) fn new(root: &'a mut Node) -> Self {
        Self {
            root: Some(root),
            traversal_stack: Vec::new(),
            item_stack: Vec::new(),
        }
    }

    #[doc = include_str!("../../doc_files/ancestors_leaves.md")]
    pub fn leaves(
        self,
    ) -> MutBorrowedDFSLeavesPostorderIteratorWithAncestors<
        'a,
        Node,
        <Node::MutBorrowedChildren as IntoIterator>::IntoIter,
    > {
        MutBorrowedDFSLeavesPostorderIteratorWithAncestors {
            root: self.root,
            item_stack: self.item_stack,
            old_traversal_stack: self.traversal_stack,
            new_traversal_stack: Vec::new(),
        }
    }
}

impl<'a, Node> StreamingIterator for MutBorrowedDFSPreorderIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    type Item = [Node::MutBorrowedValue];
    preorder_ancestors_streaming_iterator_impl!(get_value_and_children_iter_mut);
}

impl<'a, Node> StreamingIteratorMut for MutBorrowedDFSPreorderIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    get_mut_ancestors!();
}

crate::collection_iterators::mut_borrowed_collection_iterator_impl!(
    MutBorrowedBinaryDFSPreorderCollectionIterator,
    MutBorrowedBinaryDFSPreorderIterator,
    MutBorrowedBinaryTreeNode
);

pub(crate) struct MutBorrowedBinaryDFSPreorderCollectionIteratorWithPathTracking<'a, Node, IntoIter>
where
    IntoIter: IntoIterator<Item = &'a mut Node>,
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    tree_collection: Enumerate<Fuse<IntoIter::IntoIter>>,
    current_tree_iterator: Option<MutBorrowedBinaryDFSPreorderIteratorWithPathTracking<'a, Node>>,
}

impl<'a, Node, IntoIter>
    MutBorrowedBinaryDFSPreorderCollectionIteratorWithPathTracking<'a, Node, IntoIter>
where
    IntoIter: IntoIterator<Item = &'a mut Node>,
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    pub(crate) fn new(into_iter: IntoIter) -> Self {
        Self {
            tree_collection: into_iter.into_iter().fuse().enumerate(),
            current_tree_iterator: None,
        }
    }
}

impl<'a, Node, IntoIter> Iterator
    for MutBorrowedBinaryDFSPreorderCollectionIteratorWithPathTracking<'a, Node, IntoIter>
where
    IntoIter: IntoIterator<Item = &'a mut Node>,
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    type Item = Node::MutBorrowedValue;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(current_tree_iterator) = &mut self.current_tree_iterator {
                if let Some(current) = current_tree_iterator.next() {
                    return Some(current);
                }
            }

            if let Some(next_tree) = self.tree_collection.next() {
                let mut path = Vec::new();
                path.push(next_tree.0);
                self.current_tree_iterator = Some(
                    MutBorrowedBinaryDFSPreorderIteratorWithPathTracking::new(next_tree.1, path),
                );
            } else {
                return None;
            }
        }
    }
}

impl<'a, Node, IntoIter>
    TreeCollectionIteratorBase<Node::MutBorrowedValue, [Option<&'a mut Node>; 2]>
    for MutBorrowedBinaryDFSPreorderCollectionIteratorWithPathTracking<'a, Node, IntoIter>
where
    IntoIter: IntoIterator<Item = &'a mut Node>,
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    fn current_path(&self) -> &[usize] {
        self.current_tree_iterator
            .as_ref()
            .map(|iter| iter.current_path())
            .unwrap_or(&[])
    }

    fn prune_current_subtree(&mut self) {
        if let Some(inner) = self.current_tree_iterator.as_mut() {
            inner.prune_current_subtree();
        }
    }
}

impl<'a, Node, IntoIter>
    BinaryTreeCollectionIterator<Node::MutBorrowedValue, [Option<&'a mut Node>; 2]>
    for MutBorrowedBinaryDFSPreorderCollectionIteratorWithPathTracking<'a, Node, IntoIter>
where
    IntoIter: IntoIterator<Item = &'a mut Node>,
    Node: MutBorrowedBinaryTreeNode<'a>,
{
}

impl<'a, IntoIter, Node> MutBorrowedBinaryDFSPreorderCollectionIterator<'a, IntoIter, Node>
where
    IntoIter: IntoIterator<Item = &'a mut Node>,
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    #[doc = include_str!("../../doc_files/collection_attach_context.md")]
    pub fn attach_context(
        self,
    ) -> MutBorrowedBinaryDFSPreorderCollectionIteratorWithContext<'a, IntoIter, Node> {
        MutBorrowedBinaryDFSPreorderCollectionIteratorWithContext::new(self)
    }
}

crate::collection_iterators::mut_borrowed_binary_collection_context_iterator_impl!(
    MutBorrowedBinaryDFSPreorderCollectionIteratorWithContext,
    MutBorrowedBinaryDFSPreorderIteratorWithContext,
    MutBorrowedBinaryDFSPreorderCollectionIterator
);

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
    pub(crate) fn new(root: &'a mut Node) -> Self {
        Self {
            root: Some(root),
            traversal_stack: Vec::new(),
        }
    }

    #[doc = include_str!("../../doc_files/leaves.md")]
    pub fn leaves(self) -> MutBorrowedBinaryLeavesIterator<'a, Node, BinaryChildren<&'a mut Node>> {
        MutBorrowedBinaryLeavesIterator {
            root: self.root,
            traversal_stack_bottom: self.traversal_stack,
            traversal_stack_top: Vec::new(),
            item_stack: Vec::new(),
        }
    }

    #[doc = include_str!("../../doc_files/attach_context.md")]
    pub fn attach_context(self) -> MutBorrowedBinaryDFSPreorderIteratorWithContext<'a, Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS preorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                MutBorrowedBinaryDFSPreorderIteratorWithContext::new(root, Vec::new())
            }
        }
    }

    #[doc = include_str!("../../doc_files/attach_ancestors.md")]
    pub fn attach_ancestors(self) -> MutBorrowedBinaryDFSPreorderIteratorWithAncestors<'a, Node> {
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

pub(crate) struct MutBorrowedBinaryDFSPreorderIteratorWithPathTracking<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    root: Option<&'a mut Node>,
    traversal_stack: Vec<IntoIter<Option<&'a mut Node>, 2>>,
    path: Vec<usize>,
    on_deck_into_iterator: Option<[Option<&'a mut Node>; 2]>,
}

impl<'a, Node> MutBorrowedBinaryDFSPreorderIteratorWithPathTracking<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    pub(crate) fn new(root: &'a mut Node, path: Vec<usize>) -> Self {
        Self {
            root: Some(root),
            traversal_stack: Vec::new(),
            path,
            on_deck_into_iterator: None,
        }
    }
}

impl<'a, Node> Iterator for MutBorrowedBinaryDFSPreorderIteratorWithPathTracking<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    type Item = Node::MutBorrowedValue;
    dfs_preorder_binary_next_with_path_tracking!(get_value_and_children_binary_iter_mut);
}

impl<'a, Node> TreeIteratorBase<Node::MutBorrowedValue, [Option<&'a mut Node>; 2]>
    for MutBorrowedBinaryDFSPreorderIteratorWithPathTracking<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    fn current_path(&self) -> &[usize] {
        &self.path
    }

    fn prune_current_subtree(&mut self) {
        self.on_deck_into_iterator.take();
    }
}

impl<'a, Node> BinaryTreeIterator<Node::MutBorrowedValue, [Option<&'a mut Node>; 2]>
    for MutBorrowedBinaryDFSPreorderIteratorWithPathTracking<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
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

    #[doc = include_str!("../../doc_files/ancestors_leaves.md")]
    pub fn leaves(
        self,
    ) -> MutBorrowedBinaryDFSLeavesPostorderIteratorWithAncestors<
        'a,
        Node,
        BinaryChildren<&'a mut Node>,
    > {
        MutBorrowedBinaryDFSLeavesPostorderIteratorWithAncestors {
            root: self.root,
            item_stack: self.item_stack,
            old_traversal_stack: self.traversal_stack,
            new_traversal_stack: Vec::new(),
        }
    }
}

impl<'a, Node> StreamingIterator for MutBorrowedBinaryDFSPreorderIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    type Item = [Node::MutBorrowedValue];
    preorder_ancestors_streaming_iterator_impl!(get_value_and_children_iter_mut);
}

impl<'a, Node> StreamingIteratorMut for MutBorrowedBinaryDFSPreorderIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    get_mut_ancestors!();
}

pub struct MutBorrowedBinaryDFSPreorderIteratorWithContext<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    root: Option<&'a mut Node>,
    traversal_stack: Vec<IntoIter<Option<&'a mut Node>, 2>>,
    current_context: TreeContext<Node::MutBorrowedValue, [Option<&'a mut Node>; 2]>,
}

impl<'a, Node> MutBorrowedBinaryDFSPreorderIteratorWithContext<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    pub(crate) fn new(root: &'a mut Node, path: Vec<usize>) -> Self {
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

impl<'a, Node> StreamingIterator for MutBorrowedBinaryDFSPreorderIteratorWithContext<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    type Item = TreeContext<Node::MutBorrowedValue, [Option<&'a mut Node>; 2]>;
    preorder_binary_context_streaming_iterator_impl!(get_value_and_children_binary_iter_mut);
}

impl<'a, Node> StreamingIteratorMut for MutBorrowedBinaryDFSPreorderIteratorWithContext<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    get_mut_context!();
}
