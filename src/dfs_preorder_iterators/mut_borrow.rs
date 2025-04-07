use core::array::IntoIter;

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
        BinaryChildren, BinaryTreeContextIterator, MutBorrowedBinaryTreeNode, MutBorrowedTreeNode, TreeContext, TreeContextIterator, TreeContextIteratorBase
    },
};

use super::{
    dfs_preorder_next, get_mut_ancestors, get_mut_context,
    preorder_ancestors_streaming_iterator_impl, preorder_binary_context_streaming_iterator_impl,
    preorder_context_streaming_iterator_impl,
};

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
                MutBorrowedDFSPreorderIteratorWithContext::new(root)
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
    pub(crate) fn new(root: &'a mut Node) -> Self {
        Self {
            root: Some(root),
            traversal_stack: Vec::new(),
            current_context: TreeContext::new(),
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

impl<'a, Node> crate::Sealed for MutBorrowedDFSPreorderIteratorWithContext<'a, Node> where
    Node: MutBorrowedTreeNode<'a>
{
}

impl<'a, Node> TreeContextIteratorBase<Node::MutBorrowedValue, Node::MutBorrowedChildren>
    for MutBorrowedDFSPreorderIteratorWithContext<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    fn prune_current_subtree(&mut self) {
        self.current_context.children.take();
    }
}

impl<'a, Node> TreeContextIterator<Node::MutBorrowedValue, Node::MutBorrowedChildren>
    for MutBorrowedDFSPreorderIteratorWithContext<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
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
                MutBorrowedBinaryDFSPreorderIteratorWithContext::new(root)
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
    pub(crate) fn new(root: &'a mut Node) -> Self {
        Self {
            root: Some(root),
            traversal_stack: Vec::new(),
            current_context: TreeContext::new(),
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

impl<'a, Node> crate::Sealed for MutBorrowedBinaryDFSPreorderIteratorWithContext<'a, Node> where
    Node: MutBorrowedBinaryTreeNode<'a>
{
}

impl<'a, Node> TreeContextIteratorBase<Node::MutBorrowedValue, [Option<&'a mut Node>; 2]>
    for MutBorrowedBinaryDFSPreorderIteratorWithContext<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    fn prune_current_subtree(&mut self) {
        self.current_context.children.take();
    }
}

impl<'a, Node> BinaryTreeContextIterator<Node::MutBorrowedValue, [Option<&'a mut Node>; 2]>
    for MutBorrowedBinaryDFSPreorderIteratorWithContext<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
}
