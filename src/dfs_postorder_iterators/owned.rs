use crate::{
    leaves_iterators::{
        ancestors_depth_first::owned::{
            OwnedBinaryDFSLeavesPostorderIteratorWithAncestors,
            OwnedDFSLeavesPostorderIteratorWithAncestors,
        },
        depth_first::owned::{OwnedBinaryLeavesIterator, OwnedLeavesIterator},
    },
    prelude::{BinaryChildren, OwnedBinaryTreeNode, OwnedTreeNode},
    tree_context::TreeContext,
};
use alloc::vec::Vec;
use streaming_iterator::{StreamingIterator, StreamingIteratorMut};

use super::{
    dfs_postorder_next, get_mut, get_mut_binary, postorder_binary_streaming_iterator_impl,
    postorder_streaming_iterator_impl,
};

pub struct OwnedDFSPostorderIterator<Node>
where
    Node: OwnedTreeNode,
{
    root: Option<Node>,
    item_stack: Vec<Node::OwnedValue>,
    traversal_stack: Vec<<Node::OwnedChildren as IntoIterator>::IntoIter>,
}

impl<Node> OwnedDFSPostorderIterator<Node>
where
    Node: OwnedTreeNode,
{
    pub(crate) fn new(root: Node) -> OwnedDFSPostorderIterator<Node> {
        OwnedDFSPostorderIterator {
            root: Some(root),
            item_stack: Vec::new(),
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

    #[doc = include_str!("../../doc_files/attach_ancestors.md")]
    pub fn attach_context(self) -> OwnedDFSPostorderIteratorWithContext<Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS postorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                OwnedDFSPostorderIteratorWithContext::new(root)
            }
        }
    }
}

impl<Node> Iterator for OwnedDFSPostorderIterator<Node>
where
    Node: OwnedTreeNode,
{
    type Item = Node::OwnedValue;
    dfs_postorder_next!(get_value_and_children);
}

pub struct OwnedDFSPostorderIteratorWithContext<Node>
where
    Node: OwnedTreeNode,
{
    root: Option<Node>,
    traversal_stack: Vec<<Node::OwnedChildren as IntoIterator>::IntoIter>,
    current_context: TreeContext<Node>,
}

impl<'a, Node> OwnedDFSPostorderIteratorWithContext<Node>
where
    Node: OwnedTreeNode,
{
    fn new(root: Node) -> OwnedDFSPostorderIteratorWithContext<Node> {
        Self {
            root: Some(root),
            traversal_stack: Vec::new(),
            current_context: TreeContext::new(),
        }
    }

    #[doc = include_str!("../../doc_files/ancestors_leaves.md")]
    pub fn leaves(
        mut self,
    ) -> OwnedDFSLeavesPostorderIteratorWithAncestors<
        Node,
        <Node::OwnedChildren as IntoIterator>::IntoIter,
    > {
        if let Some(children) = self.current_context.children {
            self.traversal_stack.push(children.into_iter());
        }

        OwnedDFSLeavesPostorderIteratorWithAncestors {
            root: self.root,
            item_stack: self.current_context.ancestors,
            old_traversal_stack: self.traversal_stack.into_iter().collect(),
            new_traversal_stack: Vec::new(),
        }
    }
}

impl<Node> StreamingIterator for OwnedDFSPostorderIteratorWithContext<Node>
where
    Node: OwnedTreeNode,
{
    type Item = TreeContext<Node>;
    postorder_streaming_iterator_impl!(get_value_and_children);
}

impl<Node> StreamingIteratorMut for OwnedDFSPostorderIteratorWithContext<Node>
where
    Node: OwnedTreeNode,
{
    get_mut!();
}

pub struct OwnedBinaryDFSPostorderIterator<Node>
where
    Node: OwnedBinaryTreeNode,
{
    root: Option<Node>,
    item_stack: Vec<Node::OwnedValue>,
    traversal_stack: Vec<<BinaryChildren<Node> as IntoIterator>::IntoIter>,
}

impl<Node> OwnedBinaryDFSPostorderIterator<Node>
where
    Node: OwnedBinaryTreeNode,
{
    pub(crate) fn new(root: Node) -> OwnedBinaryDFSPostorderIterator<Node> {
        OwnedBinaryDFSPostorderIterator {
            root: Some(root),
            item_stack: Vec::new(),
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

    #[doc = include_str!("../../doc_files/attach_ancestors.md")]
    pub fn attach_ancestors(self) -> OwnedBinaryDFSPostorderIteratorWithAncestors<Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS postorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                OwnedBinaryDFSPostorderIteratorWithAncestors::new(root)
            }
        }
    }
}

impl<Node> Iterator for OwnedBinaryDFSPostorderIterator<Node>
where
    Node: OwnedBinaryTreeNode,
{
    type Item = Node::OwnedValue;
    dfs_postorder_next!(get_value_and_children);
}

pub struct OwnedBinaryDFSPostorderIteratorWithAncestors<Node>
where
    Node: OwnedBinaryTreeNode,
{
    root: Option<Node>,
    item_stack: Vec<Node::OwnedValue>,
    traversal_stack: Vec<BinaryChildren<Node>>,
}

impl<'a, Node> OwnedBinaryDFSPostorderIteratorWithAncestors<Node>
where
    Node: OwnedBinaryTreeNode,
{
    fn new(root: Node) -> OwnedBinaryDFSPostorderIteratorWithAncestors<Node> {
        Self {
            root: Some(root),
            item_stack: Vec::new(),
            traversal_stack: Vec::new(),
        }
    }

    #[doc = include_str!("../../doc_files/ancestors_leaves.md")]
    pub fn leaves(
        self,
    ) -> OwnedBinaryDFSLeavesPostorderIteratorWithAncestors<Node, BinaryChildren<Node>> {
        OwnedBinaryDFSLeavesPostorderIteratorWithAncestors {
            root: self.root,
            item_stack: self.item_stack,
            old_traversal_stack: self.traversal_stack.into_iter().collect(),
            new_traversal_stack: Vec::new(),
        }
    }
}

impl<Node> StreamingIterator for OwnedBinaryDFSPostorderIteratorWithAncestors<Node>
where
    Node: OwnedBinaryTreeNode,
{
    type Item = [Node::OwnedValue];
    postorder_binary_streaming_iterator_impl!(get_value_and_children);
}

impl<Node> StreamingIteratorMut for OwnedBinaryDFSPostorderIteratorWithAncestors<Node>
where
    Node: OwnedBinaryTreeNode,
{
    get_mut_binary!();
}
