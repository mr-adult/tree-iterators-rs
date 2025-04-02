use core::array::IntoIter;

use crate::{
    leaves_iterators::{
        ancestors_depth_first::borrow::{
            BorrowedBinaryDFSLeavesPostorderIteratorWithAncestors,
            BorrowedDFSLeavesPostorderIteratorWithAncestors,
        },
        depth_first::borrow::{BorrowedBinaryLeavesIterator, BorrowedLeavesIterator},
    },
    prelude::{BinaryChildren, BinaryTreeContextRef, BorrowedBinaryTreeNode, BorrowedTreeNode},
    tree_context::TreeContextRef,
};
use alloc::vec::Vec;
use streaming_iterator::StreamingIterator;

use super::{
    dfs_preorder_next, preorder_ancestors_streaming_iterator_impl,
    preorder_binary_context_streaming_iterator_impl, preorder_context_streaming_iterator_impl,
};

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
                BorrowedDFSPreorderIteratorWithContext::new(root)
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

pub struct BorrowedDFSPreorderIteratorWithContext<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    root: Option<&'a Node>,
    traversal_stack: Vec<<Node::BorrowedChildren as IntoIterator>::IntoIter>,
    current_context: TreeContextRef<'a, Node>,
}

impl<'a, Node> BorrowedDFSPreorderIteratorWithContext<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    pub(crate) fn new(root: &'a Node) -> Self {
        Self {
            root: Some(root),
            traversal_stack: Vec::new(),
            current_context: TreeContextRef::new(),
        }
    }

    #[doc = include_str!("../../doc_files/ancestors_leaves.md")]
    pub fn leaves(
        mut self,
    ) -> BorrowedDFSLeavesPostorderIteratorWithAncestors<
        'a,
        Node,
        <Node::BorrowedChildren as IntoIterator>::IntoIter,
    > {
        if self.root.is_none() && !self.is_done() {
            self.traversal_stack
                .push(unsafe { self.current_context.children.assume_init() }.into_iter());
        }

        BorrowedDFSLeavesPostorderIteratorWithAncestors {
            root: self.root,
            item_stack: self.current_context.ancestors,
            old_traversal_stack: self.traversal_stack,
            new_traversal_stack: Vec::new(),
        }
    }
}

impl<'a, Node> StreamingIterator for BorrowedDFSPreorderIteratorWithContext<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    type Item = TreeContextRef<'a, Node>;
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
                BorrowedBinaryDFSPreorderIteratorWithContext::new(root)
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
    current_context: BinaryTreeContextRef<'a, Node>,
}

impl<'a, Node> BorrowedBinaryDFSPreorderIteratorWithContext<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    pub(crate) fn new(root: &'a Node) -> Self {
        Self {
            root: Some(root),
            traversal_stack: Vec::new(),
            current_context: BinaryTreeContextRef::new(),
        }
    }

    #[doc = include_str!("../../doc_files/ancestors_leaves.md")]
    pub fn leaves(
        self,
    ) -> BorrowedBinaryDFSLeavesPostorderIteratorWithAncestors<'a, Node, IntoIter<&'a Node, 2>>
    {
        todo!();
    }
}

impl<'a, Node> StreamingIterator for BorrowedBinaryDFSPreorderIteratorWithContext<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    type Item = BinaryTreeContextRef<'a, Node>;
    preorder_binary_context_streaming_iterator_impl!(get_value_and_children_binary_iter);
}
