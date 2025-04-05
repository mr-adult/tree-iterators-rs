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
    prelude::{BinaryChildren, BinaryTreeContext, OwnedBinaryTreeNode, OwnedTreeNode, TreeContext},
};

use super::{
    dfs_preorder_next, get_mut_ancestors, get_mut_context,
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

pub struct OwnedDFSPreorderIteratorWithContext<Node>
where
    Node: OwnedTreeNode,
{
    root: Option<Node>,
    traversal_stack: Vec<<Node::OwnedChildren as IntoIterator>::IntoIter>,
    current_context: TreeContext<Node>,
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
    type Item = TreeContext<Node>;
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
    traversal_stack: Vec<BinaryChildren<Node>>,
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
    current_context: BinaryTreeContext<Node>,
}

impl<Node> OwnedBinaryDFSPreorderIteratorWithContext<Node>
where
    Node: OwnedBinaryTreeNode,
{
    pub(crate) fn new(root: Node) -> Self {
        Self {
            root: Some(root),
            traversal_stack: Vec::new(),
            current_context: BinaryTreeContext::new(),
        }
    }
}

impl<Node> StreamingIterator for OwnedBinaryDFSPreorderIteratorWithContext<Node>
where
    Node: OwnedBinaryTreeNode,
{
    type Item = BinaryTreeContext<Node>;
    preorder_binary_context_streaming_iterator_impl!(get_value_and_children_binary);
}

impl<Node> StreamingIteratorMut for OwnedBinaryDFSPreorderIteratorWithContext<Node>
where
    Node: OwnedBinaryTreeNode,
{
    get_mut_context!();
}
