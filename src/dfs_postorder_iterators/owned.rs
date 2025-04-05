use core::array::IntoIter;

use crate::{
    leaves_iterators::{
        ancestors_depth_first::owned::{
            OwnedBinaryDFSLeavesPostorderIteratorWithAncestors,
            OwnedDFSLeavesPostorderIteratorWithAncestors,
        },
        depth_first::owned::{OwnedBinaryLeavesIterator, OwnedLeavesIterator},
    },
    prelude::{BinaryChildren, OwnedBinaryTreeNode, OwnedTreeNode, TreeContext},
};
use alloc::vec::Vec;
use streaming_iterator::{StreamingIterator, StreamingIteratorMut};

use super::{
    dfs_postorder_next, get_mut_ancestors, get_mut_context,
    postorder_ancestors_streaming_iterator_impl,
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

    #[doc = include_str!("../../doc_files/attach_context.md")]
    pub fn attach_context(self) -> OwnedDFSPostorderIteratorWithContext<Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS postorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                OwnedDFSPostorderIteratorWithContext::new(root)
            }
        }
    }

    #[doc = include_str!("../../doc_files/attach_ancestors.md")]
    pub fn attach_ancestors(self) -> OwnedDFSPostorderIteratorWithAncestors<Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS postorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                OwnedDFSPostorderIteratorWithAncestors::new(root)
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
    current_context: TreeContext<Node::OwnedValue, ()>,
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
}

impl<Node> StreamingIterator for OwnedDFSPostorderIteratorWithContext<Node>
where
    Node: OwnedTreeNode,
{
    type Item = TreeContext<Node::OwnedValue, ()>;
    fn advance(&mut self) {
        if let Some(next) = self.root.take() {
            let (value, children) = next.get_value_and_children();
            self.traversal_stack.push(children.into_iter());
            self.current_context.ancestors.push(value);
            self.current_context.path.push(usize::MAX);
        } else {
            self.current_context.ancestors.pop();
            if self.current_context.ancestors.is_empty() {
                return;
            }
        }

        loop {
            if let Some(top) = self.traversal_stack.last_mut() {
                if let Some(node) = top.next() {
                    let last = self
                        .current_context
                        .path
                        .last_mut()
                        .expect("There to be a path unless we are on the root element");
                    *last = last.wrapping_add(1);

                    let (value, children) = node.get_value_and_children();

                    self.traversal_stack.push(children.into_iter());
                    self.current_context.ancestors.push(value);
                    self.current_context.path.push(usize::MAX);
                    continue;
                }
            }

            self.traversal_stack.pop();
            self.current_context.path.pop();
            return;
        }
    }

    fn get(&self) -> Option<&Self::Item> {
        if self.current_context.ancestors.is_empty() {
            None
        } else {
            Some(&self.current_context)
        }
    }
}

impl<Node> StreamingIteratorMut for OwnedDFSPostorderIteratorWithContext<Node>
where
    Node: OwnedTreeNode,
{
    get_mut_context!();
}

pub struct OwnedDFSPostorderIteratorWithAncestors<Node>
where
    Node: OwnedTreeNode,
{
    root: Option<Node>,
    item_stack: Vec<Node::OwnedValue>,
    traversal_stack: Vec<<Node::OwnedChildren as IntoIterator>::IntoIter>,
}

impl<'a, Node> OwnedDFSPostorderIteratorWithAncestors<Node>
where
    Node: OwnedTreeNode,
{
    fn new(root: Node) -> OwnedDFSPostorderIteratorWithAncestors<Node> {
        Self {
            root: Some(root),
            item_stack: Vec::new(),
            traversal_stack: Vec::new(),
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
            old_traversal_stack: self.traversal_stack.into_iter().collect(),
            new_traversal_stack: Vec::new(),
        }
    }
}

impl<Node> StreamingIterator for OwnedDFSPostorderIteratorWithAncestors<Node>
where
    Node: OwnedTreeNode,
{
    type Item = [Node::OwnedValue];
    postorder_ancestors_streaming_iterator_impl!(get_value_and_children);
}

impl<Node> StreamingIteratorMut for OwnedDFSPostorderIteratorWithAncestors<Node>
where
    Node: OwnedTreeNode,
{
    get_mut_ancestors!();
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

    #[doc = include_str!("../../doc_files/attach_context.md")]
    pub fn attach_context(self) -> OwnedBinaryDFSPostorderIteratorWithContext<Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS postorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                OwnedBinaryDFSPostorderIteratorWithContext::new(root)
            }
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
    postorder_ancestors_streaming_iterator_impl!(get_value_and_children);
}

impl<Node> StreamingIteratorMut for OwnedBinaryDFSPostorderIteratorWithAncestors<Node>
where
    Node: OwnedBinaryTreeNode,
{
    get_mut_ancestors!();
}

pub struct OwnedBinaryDFSPostorderIteratorWithContext<Node>
where
    Node: OwnedBinaryTreeNode,
{
    root: Option<Node>,
    traversal_stack: Vec<IntoIter<Option<Node>, 2>>,
    current_context: TreeContext<Node::OwnedValue, ()>,
}

impl<'a, Node> OwnedBinaryDFSPostorderIteratorWithContext<Node>
where
    Node: OwnedBinaryTreeNode,
{
    fn new(root: Node) -> OwnedBinaryDFSPostorderIteratorWithContext<Node> {
        Self {
            root: Some(root),
            current_context: TreeContext::new(),
            traversal_stack: Vec::new(),
        }
    }
}

impl<'a, Node> StreamingIterator for OwnedBinaryDFSPostorderIteratorWithContext<Node>
where
    Node: OwnedBinaryTreeNode,
{
    type Item = TreeContext<Node::OwnedValue, ()>;
    fn advance(&mut self) {
        if let Some(next) = self.root.take() {
            let (value, children) = next.get_value_and_children_binary();
            self.traversal_stack.push(children.into_iter());
            self.current_context.ancestors.push(value);
            self.current_context.path.push(usize::MAX);
        } else {
            self.current_context.ancestors.pop();
            if self.current_context.ancestors.is_empty() {
                return;
            }
        }

        'outer: loop {
            if let Some(top) = self.traversal_stack.last_mut() {
                while let Some(node) = top.next() {
                    let last = self
                        .current_context
                        .path
                        .last_mut()
                        .expect("There to be a path unless we are on the root element");
                    *last = last.wrapping_add(1);

                    if let Some(node) = node {
                        // Path is not populated on the first pass over just the root node.

                        let (value, children) = node.get_value_and_children_binary();

                        self.traversal_stack.push(children.into_iter());
                        self.current_context.ancestors.push(value);
                        self.current_context.path.push(usize::MAX);
                        continue 'outer;
                    }
                }
            }

            self.traversal_stack.pop();
            self.current_context.path.pop();
            return;
        }
    }

    fn get(&self) -> Option<&Self::Item> {
        if self.current_context.ancestors.is_empty() {
            None
        } else {
            Some(&self.current_context)
        }
    }
}
