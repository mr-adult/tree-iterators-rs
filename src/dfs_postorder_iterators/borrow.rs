use crate::{
    leaves_iterators::{
        ancestors_depth_first::borrow::{
            BorrowedBinaryDFSLeavesPostorderIteratorWithAncestors,
            BorrowedDFSLeavesPostorderIteratorWithAncestors,
        },
        depth_first::borrow::{BorrowedBinaryLeavesIterator, BorrowedLeavesIterator},
    },
    prelude::{BinaryChildren, BorrowedBinaryTreeNode, BorrowedTreeNode},
    tree_context::TreeContextRef,
};
use alloc::vec::Vec;
use streaming_iterator::StreamingIterator;

use super::{
    dfs_postorder_next, postorder_binary_streaming_iterator_impl, postorder_streaming_iterator_impl,
};

pub struct BorrowedDFSPostorderIterator<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    root: Option<&'a Node>,
    item_stack: Vec<Node::BorrowedValue>,
    traversal_stack: Vec<<Node::BorrowedChildren as IntoIterator>::IntoIter>,
}

impl<'a, Node> BorrowedDFSPostorderIterator<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    pub(crate) fn new(root: &'a Node) -> BorrowedDFSPostorderIterator<'a, Node> {
        BorrowedDFSPostorderIterator {
            root: Some(root),
            item_stack: Vec::new(),
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

    #[doc = include_str!("../../doc_files/attach_ancestors.md")]
    pub fn attach_context(self) -> BorrowedDFSPostorderIteratorWithContext<'a, Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS postorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                BorrowedDFSPostorderIteratorWithContext::new(root)
            }
        }
    }
}

impl<'a, Node> Iterator for BorrowedDFSPostorderIterator<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    type Item = Node::BorrowedValue;
    dfs_postorder_next!(get_value_and_children_iter);
}

pub struct BorrowedDFSPostorderIteratorWithContext<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    root: Option<&'a Node>,
    traversal_stack: Vec<<Node::BorrowedChildren as IntoIterator>::IntoIter>,
    current_context: TreeContextRef<'a, Node>,
}

impl<'a, Node> BorrowedDFSPostorderIteratorWithContext<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    fn new(root: &'a Node) -> BorrowedDFSPostorderIteratorWithContext<'_, Node> {
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
            self.traversal_stack.push(unsafe { self.current_context.children.assume_init() }.into_iter())
        }

        BorrowedDFSLeavesPostorderIteratorWithAncestors {
            root: self.root,
            item_stack: self.current_context.ancestors,
            old_traversal_stack: self.traversal_stack.into_iter().collect(),
            new_traversal_stack: Vec::new(),
        }
    }
}

impl<'a, Node> StreamingIterator for BorrowedDFSPostorderIteratorWithContext<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    type Item = TreeContextRef<'a, Node>;
    postorder_streaming_iterator_impl!(get_value_and_children_iter);
}

pub struct BorrowedBinaryDFSPostorderIterator<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    root: Option<&'a Node>,
    item_stack: Vec<Node::BorrowedValue>,
    traversal_stack: Vec<<BinaryChildren<&'a Node> as IntoIterator>::IntoIter>,
}

impl<'a, Node> BorrowedBinaryDFSPostorderIterator<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    pub(crate) fn new(root: &'a Node) -> BorrowedBinaryDFSPostorderIterator<'a, Node> {
        BorrowedBinaryDFSPostorderIterator {
            root: Some(root),
            item_stack: Vec::new(),
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

    #[doc = include_str!("../../doc_files/attach_ancestors.md")]
    pub fn attach_context(self) -> BorrowedBinaryDFSPostorderIteratorWithAncestors<'a, Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS postorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                BorrowedBinaryDFSPostorderIteratorWithAncestors::new(root)
            }
        }
    }
}

impl<'a, Node> Iterator for BorrowedBinaryDFSPostorderIterator<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    type Item = Node::BorrowedValue;
    dfs_postorder_next!(get_value_and_children_iter);
}

pub struct BorrowedBinaryDFSPostorderIteratorWithAncestors<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    root: Option<&'a Node>,
    item_stack: Vec<Node::BorrowedValue>,
    traversal_stack: Vec<BinaryChildren<&'a Node>>,
}

impl<'a, Node> BorrowedBinaryDFSPostorderIteratorWithAncestors<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    fn new(root: &'a Node) -> BorrowedBinaryDFSPostorderIteratorWithAncestors<'_, Node> {
        Self {
            root: Some(root),
            item_stack: Vec::new(),
            traversal_stack: Vec::new(),
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
            old_traversal_stack: self.traversal_stack.into_iter().collect(),
            new_traversal_stack: Vec::new(),
        }
    }
}

impl<'a, Node> StreamingIterator for BorrowedBinaryDFSPostorderIteratorWithAncestors<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    type Item = [Node::BorrowedValue];
    postorder_binary_streaming_iterator_impl!(get_value_and_children_iter);
}
