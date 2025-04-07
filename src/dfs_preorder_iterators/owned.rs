use core::array::IntoIter;

use alloc::vec::Vec;
use streaming_iterator::{StreamingIterator, StreamingIteratorMut};

use crate::{
    leaves_iterators::{
        ancestors_depth_first::owned::{
            OwnedBinaryDFSLeavesPostorderIteratorWithAncestors,
            OwnedDFSLeavesPostorderIteratorWithAncestors,
        },
        depth_first::owned::{OwnedBinaryLeavesIterator, OwnedLeavesIterator},
    },
    prelude::{
        BinaryChildren, BinaryTreeIterator, OwnedBinaryTreeNode, OwnedTreeNode, TreeContext,
    },
    tree_iterators::{TreeIterator, TreeIteratorBase},
};

use super::{
    dfs_preorder_binary_next_with_path_tracking, dfs_preorder_next,
    dfs_preorder_next_with_path_tracking, get_mut_ancestors, get_mut_context,
    preorder_ancestors_streaming_iterator_impl, preorder_binary_context_streaming_iterator_impl,
    preorder_context_streaming_iterator_impl,
};

pub struct OwnedDFSPreorderIterator<Node>
where
    Node: OwnedTreeNode,
{
    root: Option<Node>,
    traversal_stack: Vec<<Node::OwnedChildren as IntoIterator>::IntoIter>,
}

impl<Node> OwnedDFSPreorderIterator<Node>
where
    Node: OwnedTreeNode,
{
    pub(crate) fn new(root: Node) -> Self {
        Self {
            root: Some(root),
            traversal_stack: Vec::new(),
        }
    }

    #[doc = include_str!("../../doc_files/leaves.md")]
    pub fn leaves(self) -> OwnedLeavesIterator<Node> {
        OwnedLeavesIterator {
            root: self.root,
            traversal_stack_bottom: self.traversal_stack,
            traversal_stack_top: Vec::new(),
            item_stack: Vec::new(),
        }
    }

    #[doc = include_str!("../../doc_files/attach_context.md")]
    pub fn attach_context(self) -> OwnedDFSPreorderIteratorWithContext<Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS preorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                OwnedDFSPreorderIteratorWithContext::new(root)
            }
        }
    }

    #[doc = include_str!("../../doc_files/attach_ancestors.md")]
    pub fn attach_ancestors(self) -> OwnedDFSPreorderIteratorWithAncestors<Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS preorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                OwnedDFSPreorderIteratorWithAncestors::new(root)
            }
        }
    }
}

impl<Node> Iterator for OwnedDFSPreorderIterator<Node>
where
    Node: OwnedTreeNode,
{
    type Item = Node::OwnedValue;
    dfs_preorder_next!(get_value_and_children);
}

pub(crate) struct OwnedDFSPreorderIteratorWithPathTracking<Node>
where
    Node: OwnedTreeNode,
{
    root: Option<Node>,
    path: Vec<usize>,
    on_deck_into_iterator: Option<Node::OwnedChildren>,
    traversal_stack: Vec<<Node::OwnedChildren as IntoIterator>::IntoIter>,
}

impl<Node> OwnedDFSPreorderIteratorWithPathTracking<Node>
where
    Node: OwnedTreeNode,
{
    pub(crate) fn new(root: Node) -> Self {
        Self {
            root: Some(root),
            path: Vec::new(),
            on_deck_into_iterator: None,
            traversal_stack: Vec::new(),
        }
    }
}

impl<Node> Iterator for OwnedDFSPreorderIteratorWithPathTracking<Node>
where
    Node: OwnedTreeNode,
{
    type Item = Node::OwnedValue;
    dfs_preorder_next_with_path_tracking!(get_value_and_children);
}

impl<Node> crate::Sealed for OwnedDFSPreorderIteratorWithPathTracking<Node> where Node: OwnedTreeNode
{}

impl<Node> TreeIteratorBase<Node::OwnedValue, Node::OwnedChildren>
    for OwnedDFSPreorderIteratorWithPathTracking<Node>
where
    Node: OwnedTreeNode,
{
    fn current_path(&self) -> &[usize] {
        &self.path
    }

    fn prune_current_subtree(&mut self) {
        self.on_deck_into_iterator.take();
    }
}

impl<Node> TreeIterator<Node::OwnedValue, Node::OwnedChildren>
    for OwnedDFSPreorderIteratorWithPathTracking<Node>
where
    Node: OwnedTreeNode,
{
}

pub struct OwnedDFSPreorderIteratorWithContext<Node>
where
    Node: OwnedTreeNode,
{
    root: Option<Node>,
    traversal_stack: Vec<<Node::OwnedChildren as IntoIterator>::IntoIter>,
    current_context: TreeContext<Node::OwnedValue, Node::OwnedChildren>,
}

impl<'a, Node> OwnedDFSPreorderIteratorWithContext<Node>
where
    Node: OwnedTreeNode,
{
    pub(crate) fn new(root: Node) -> Self {
        Self {
            root: Some(root),
            traversal_stack: Vec::new(),
            current_context: TreeContext::new(),
        }
    }
}

impl<Node> StreamingIterator for OwnedDFSPreorderIteratorWithContext<Node>
where
    Node: OwnedTreeNode,
{
    type Item = TreeContext<Node::OwnedValue, Node::OwnedChildren>;
    preorder_context_streaming_iterator_impl!(get_value_and_children);
}

impl<Node> StreamingIteratorMut for OwnedDFSPreorderIteratorWithContext<Node>
where
    Node: OwnedTreeNode,
{
    get_mut_context!();
}

pub struct OwnedDFSPreorderIteratorWithAncestors<Node>
where
    Node: OwnedTreeNode,
{
    root: Option<Node>,
    traversal_stack: Vec<<Node::OwnedChildren as IntoIterator>::IntoIter>,
    item_stack: Vec<Node::OwnedValue>,
}

impl<'a, Node> OwnedDFSPreorderIteratorWithAncestors<Node>
where
    Node: OwnedTreeNode,
{
    pub(crate) fn new(root: Node) -> Self {
        Self {
            root: Some(root),
            traversal_stack: Vec::new(),
            item_stack: Vec::new(),
        }
    }

    #[doc = include_str!("../../doc_files/ancestors_leaves.md")]
    pub fn leaves(
        self,
    ) -> OwnedDFSLeavesPostorderIteratorWithAncestors<
        Node,
        <Node::OwnedChildren as IntoIterator>::IntoIter,
    > {
        OwnedDFSLeavesPostorderIteratorWithAncestors {
            root: self.root,
            item_stack: self.item_stack,
            old_traversal_stack: self.traversal_stack,
            new_traversal_stack: Vec::new(),
        }
    }
}

impl<Node> StreamingIterator for OwnedDFSPreorderIteratorWithAncestors<Node>
where
    Node: OwnedTreeNode,
{
    type Item = [Node::OwnedValue];
    preorder_ancestors_streaming_iterator_impl!(get_value_and_children);
}

impl<Node> StreamingIteratorMut for OwnedDFSPreorderIteratorWithAncestors<Node>
where
    Node: OwnedTreeNode,
{
    get_mut_ancestors!();
}

pub struct OwnedBinaryDFSPreorderIterator<Node>
where
    Node: OwnedBinaryTreeNode,
{
    root: Option<Node>,
    pub(crate) traversal_stack: Vec<BinaryChildren<Node>>,
}

impl<Node> OwnedBinaryDFSPreorderIterator<Node>
where
    Node: OwnedBinaryTreeNode,
{
    pub(crate) fn new(root: Node) -> Self {
        Self {
            root: Some(root),
            traversal_stack: Vec::new(),
        }
    }

    #[doc = include_str!("../../doc_files/leaves.md")]
    pub fn leaves(self) -> OwnedBinaryLeavesIterator<Node, BinaryChildren<Node>> {
        OwnedBinaryLeavesIterator {
            root: self.root,
            traversal_stack_bottom: self.traversal_stack,
            traversal_stack_top: Vec::new(),
            item_stack: Vec::new(),
        }
    }

    #[doc = include_str!("../../doc_files/attach_context.md")]
    pub fn attach_context(self) -> OwnedBinaryDFSPreorderIteratorWithContext<Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS preorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                OwnedBinaryDFSPreorderIteratorWithContext::new(root)
            }
        }
    }

    #[doc = include_str!("../../doc_files/attach_ancestors.md")]
    pub fn attach_ancestors(self) -> OwnedBinaryDFSPreorderIteratorWithAncestors<Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS preorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                OwnedBinaryDFSPreorderIteratorWithAncestors::new(root)
            }
        }
    }
}

impl<Node> Iterator for OwnedBinaryDFSPreorderIterator<Node>
where
    Node: OwnedBinaryTreeNode,
{
    type Item = Node::OwnedValue;
    dfs_preorder_next!(get_value_and_children);
}

pub(crate) struct OwnedBinaryDFSPreorderIteratorWithPathTracking<Node>
where
    Node: OwnedBinaryTreeNode,
{
    root: Option<Node>,
    traversal_stack: Vec<IntoIter<Option<Node>, 2>>,
    path: Vec<usize>,
    on_deck_into_iterator: Option<[Option<Node>; 2]>,
}

impl<Node> OwnedBinaryDFSPreorderIteratorWithPathTracking<Node>
where
    Node: OwnedBinaryTreeNode,
{
    pub(crate) fn new(root: Node) -> Self {
        Self {
            root: Some(root),
            traversal_stack: Vec::new(),
            path: Vec::new(),
            on_deck_into_iterator: None,
        }
    }
}

impl<Node> Iterator for OwnedBinaryDFSPreorderIteratorWithPathTracking<Node>
where
    Node: OwnedBinaryTreeNode,
{
    type Item = Node::OwnedValue;
    dfs_preorder_binary_next_with_path_tracking!(get_value_and_children_binary);
}

impl<'a, Node> crate::Sealed for OwnedBinaryDFSPreorderIteratorWithPathTracking<Node> where
    Node: OwnedBinaryTreeNode
{
}

impl<'a, Node> TreeIteratorBase<Node::OwnedValue, [Option<Node>; 2]>
    for OwnedBinaryDFSPreorderIteratorWithPathTracking<Node>
where
    Node: OwnedBinaryTreeNode,
{
    fn current_path(&self) -> &[usize] {
        &self.path
    }

    fn prune_current_subtree(&mut self) {
        self.on_deck_into_iterator.take();
    }
}

impl<'a, Node> BinaryTreeIterator<Node::OwnedValue, [Option<Node>; 2]>
    for OwnedBinaryDFSPreorderIteratorWithPathTracking<Node>
where
    Node: OwnedBinaryTreeNode,
{
}

pub struct OwnedBinaryDFSPreorderIteratorWithAncestors<Node>
where
    Node: OwnedBinaryTreeNode,
{
    root: Option<Node>,
    traversal_stack: Vec<BinaryChildren<Node>>,
    item_stack: Vec<Node::OwnedValue>,
}

impl<'a, Node> OwnedBinaryDFSPreorderIteratorWithAncestors<Node>
where
    Node: OwnedBinaryTreeNode,
{
    pub(crate) fn new(root: Node) -> Self {
        Self {
            root: Some(root),
            traversal_stack: Vec::new(),
            item_stack: Vec::new(),
        }
    }

    #[doc = include_str!("../../doc_files/ancestors_leaves.md")]
    pub fn leaves(
        self,
    ) -> OwnedBinaryDFSLeavesPostorderIteratorWithAncestors<Node, BinaryChildren<Node>> {
        OwnedBinaryDFSLeavesPostorderIteratorWithAncestors {
            root: self.root,
            item_stack: self.item_stack,
            old_traversal_stack: self.traversal_stack,
            new_traversal_stack: Vec::new(),
        }
    }
}

impl<'a, Node> StreamingIterator for OwnedBinaryDFSPreorderIteratorWithAncestors<Node>
where
    Node: OwnedBinaryTreeNode,
{
    type Item = [Node::OwnedValue];
    preorder_ancestors_streaming_iterator_impl!(get_value_and_children);
}

impl<'a, Node> StreamingIteratorMut for OwnedBinaryDFSPreorderIteratorWithAncestors<Node>
where
    Node: OwnedBinaryTreeNode,
{
    get_mut_ancestors!();
}

pub struct OwnedBinaryDFSPreorderIteratorWithContext<Node>
where
    Node: OwnedBinaryTreeNode,
{
    root: Option<Node>,
    traversal_stack: Vec<IntoIter<Option<Node>, 2>>,
    current_context: TreeContext<Node::OwnedValue, [Option<Node>; 2]>,
}

impl<Node> OwnedBinaryDFSPreorderIteratorWithContext<Node>
where
    Node: OwnedBinaryTreeNode,
{
    pub(crate) fn new(root: Node) -> Self {
        Self {
            root: Some(root),
            traversal_stack: Vec::new(),
            current_context: TreeContext::new(),
        }
    }
}

impl<Node> StreamingIterator for OwnedBinaryDFSPreorderIteratorWithContext<Node>
where
    Node: OwnedBinaryTreeNode,
{
    type Item = TreeContext<Node::OwnedValue, [Option<Node>; 2]>;
    preorder_binary_context_streaming_iterator_impl!(get_value_and_children_binary);
}

impl<Node> StreamingIteratorMut for OwnedBinaryDFSPreorderIteratorWithContext<Node>
where
    Node: OwnedBinaryTreeNode,
{
    get_mut_context!();
}
