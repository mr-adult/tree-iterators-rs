use core::{array::IntoIter, mem::MaybeUninit};

use crate::{
    leaves_iterators::{
        ancestors_depth_first::borrow::{
            BorrowedBinaryDFSLeavesPostorderIteratorWithAncestors,
            BorrowedDFSLeavesPostorderIteratorWithAncestors,
        },
        depth_first::borrow::{BorrowedBinaryLeavesIterator, BorrowedLeavesIterator},
    },
    prelude::{BinaryChildren, BorrowedBinaryTreeNode, BorrowedTreeNode, TreeContext},
};
use alloc::vec::Vec;
use streaming_iterator::StreamingIterator;

use super::{dfs_postorder_next, postorder_ancestors_streaming_iterator_impl};

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

    #[doc = include_str!("../../doc_files/attach_context.md")]
    pub fn attach_context(self) -> BorrowedDFSPostorderIteratorWithContext<'a, Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS postorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                BorrowedDFSPostorderIteratorWithContext::new(root)
            }
        }
    }

    #[doc = include_str!("../../doc_files/attach_ancestors.md")]
    pub fn attach_ancestors(self) -> BorrowedDFSPostorderIteratorWithAncestors<'a, Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS postorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                BorrowedDFSPostorderIteratorWithAncestors::new(root)
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
    into_iterator_stack: Vec<Node::BorrowedChildren>,
    current_context: TreeContext<Node::BorrowedValue, Node::BorrowedChildren>,
}

impl<'a, Node> BorrowedDFSPostorderIteratorWithContext<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    fn new(root: &'a Node) -> BorrowedDFSPostorderIteratorWithContext<'_, Node> {
        Self {
            root: Some(root),
            traversal_stack: Vec::new(),
            into_iterator_stack: Vec::new(),
            current_context: TreeContext::new(),
        }
    }
}

impl<'a, Node> StreamingIterator for BorrowedDFSPostorderIteratorWithContext<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
    Node::BorrowedChildren: Clone,
{
    type Item = TreeContext<Node::BorrowedValue, Node::BorrowedChildren>;
    fn advance(&mut self) {
        if let Some(next) = self.root.take() {
            let (value, children) = next.get_value_and_children_iter();
            // ASSUMPTION: self.into_iterator_stack will always outlive self.traversal_stack.
            // If that assumption is not true, this code will cause Undefined Behavior.
            self.traversal_stack.push(
                unsafe { core::ptr::read(&children as *const Node::BorrowedChildren) }.into_iter(),
            );
            self.current_context.ancestors.push(value);
            self.current_context.path.push(usize::MAX);
            self.into_iterator_stack.push(children);
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

                    let (value, children) = node.get_value_and_children_iter();

                    // ASSUMPTION: self.into_iterator_stack will always outlive self.traversal_stack.
                    // If that assumption is not true, this code will cause Undefined Behavior.
                    self.traversal_stack.push(
                        unsafe { core::ptr::read(&children as *const Node::BorrowedChildren) }
                            .into_iter(),
                    );
                    self.current_context.ancestors.push(value);
                    self.current_context.path.push(usize::MAX);
                    self.into_iterator_stack.push(children);
                    continue;
                }
            }

            self.current_context.children = MaybeUninit::new(
                self.into_iterator_stack
                    .pop()
                    .expect("There to be a childe IntoIterator"),
            );
            self.traversal_stack.pop();
            self.current_context.path.pop();
            break;
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

pub struct BorrowedDFSPostorderIteratorWithAncestors<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    root: Option<&'a Node>,
    item_stack: Vec<Node::BorrowedValue>,
    traversal_stack: Vec<<Node::BorrowedChildren as IntoIterator>::IntoIter>,
}

impl<'a, Node> BorrowedDFSPostorderIteratorWithAncestors<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    fn new(root: &'a Node) -> BorrowedDFSPostorderIteratorWithAncestors<'_, Node> {
        Self {
            root: Some(root),
            item_stack: Vec::new(),
            traversal_stack: Vec::new(),
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
            old_traversal_stack: self.traversal_stack.into_iter().collect(),
            new_traversal_stack: Vec::new(),
        }
    }
}

impl<'a, Node> StreamingIterator for BorrowedDFSPostorderIteratorWithAncestors<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    type Item = [Node::BorrowedValue];
    postorder_ancestors_streaming_iterator_impl!(get_value_and_children_iter);
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

    #[doc = include_str!("../../doc_files/attach_context.md")]
    pub fn attach_context(self) -> BorrowedBinaryDFSPostorderIteratorWithContext<'a, Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS postorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                BorrowedBinaryDFSPostorderIteratorWithContext::new(root)
            }
        }
    }

    #[doc = include_str!("../../doc_files/attach_ancestors.md")]
    pub fn attach_ancestors(self) -> BorrowedBinaryDFSPostorderIteratorWithAncestors<'a, Node> {
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
    postorder_ancestors_streaming_iterator_impl!(get_value_and_children_iter);
}

pub struct BorrowedBinaryDFSPostorderIteratorWithContext<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    root: Option<&'a Node>,
    traversal_stack: Vec<IntoIter<Option<&'a Node>, 2>>,
    current_context: TreeContext<Node::BorrowedValue, [Option<&'a Node>; 2]>,
    into_iterator_stack: Vec<[Option<&'a Node>; 2]>,
}

impl<'a, Node> BorrowedBinaryDFSPostorderIteratorWithContext<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    fn new(root: &'a Node) -> BorrowedBinaryDFSPostorderIteratorWithContext<'_, Node> {
        Self {
            root: Some(root),
            current_context: TreeContext::new(),
            traversal_stack: Vec::new(),
            into_iterator_stack: Vec::new(),
        }
    }
}

impl<'a, Node> StreamingIterator for BorrowedBinaryDFSPostorderIteratorWithContext<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    type Item = TreeContext<Node::BorrowedValue, [Option<&'a Node>; 2]>;
    fn advance(&mut self) {
        if let Some(next) = self.root.take() {
            let (value, children) = next.get_value_and_children_binary_iter();
            // ASSUMPTION: self.into_iterator_stack will always outlive self.traversal_stack.
            // If that assumption is not true, this code will cause Undefined Behavior.
            self.traversal_stack.push(
                unsafe { core::ptr::read(&children as *const [Option<&'a Node>; 2]) }.into_iter(),
            );
            self.current_context.ancestors.push(value);
            self.current_context.path.push(usize::MAX);
            self.into_iterator_stack.push(children);
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
                        let (value, children) = node.get_value_and_children_binary_iter();

                        // ASSUMPTION: self.into_iterator_stack will always outlive self.traversal_stack.
                        // If that assumption is not true, this code will cause Undefined Behavior.
                        self.traversal_stack.push(
                            unsafe { core::ptr::read(&children as *const [Option<&'a Node>; 2]) }
                                .into_iter(),
                        );
                        self.current_context.ancestors.push(value);
                        self.current_context.path.push(usize::MAX);
                        self.into_iterator_stack.push(children);
                        continue 'outer;
                    }
                }
            }

            self.current_context.children = MaybeUninit::new(
                self.into_iterator_stack
                    .pop()
                    .expect("There to be a children IntoIterator"),
            );
            self.current_context.path.pop();
            self.traversal_stack.pop();
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
