use core::{array::IntoIter, mem::MaybeUninit};

use crate::{
    leaves_iterators::{
        ancestors_depth_first::mut_borrow::{
            MutBorrowedBinaryDFSLeavesPostorderIteratorWithAncestors,
            MutBorrowedDFSLeavesPostorderIteratorWithAncestors,
        },
        depth_first::mut_borrow::{MutBorrowedBinaryLeavesIterator, MutBorrowedLeavesIterator},
    },
    prelude::{
        BinaryChildren, BinaryTreeContextMut, MutBorrowedBinaryTreeNode, MutBorrowedTreeNode,
    },
    tree_context::TreeContextMut,
};
use alloc::vec::Vec;
use streaming_iterator::{StreamingIterator, StreamingIteratorMut};

use super::{
    dfs_postorder_next, get_mut_ancestors, get_mut_context,
    postorder_ancestors_streaming_iterator_impl,
};

pub struct MutBorrowedDFSPostorderIterator<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    root: Option<&'a mut Node>,
    item_stack: Vec<Node::MutBorrowedValue>,
    traversal_stack: Vec<<Node::MutBorrowedChildren as IntoIterator>::IntoIter>,
}

impl<'a, Node> MutBorrowedDFSPostorderIterator<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    pub(crate) fn new(root: &'a mut Node) -> MutBorrowedDFSPostorderIterator<'a, Node> {
        MutBorrowedDFSPostorderIterator {
            root: Some(root),
            item_stack: Vec::new(),
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
    pub fn attach_context(self) -> MutBorrowedDFSPostorderIteratorWithContext<'a, Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS postorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                MutBorrowedDFSPostorderIteratorWithContext::new(root)
            }
        }
    }

    #[doc = include_str!("../../doc_files/attach_ancestors.md")]
    pub fn attach_ancestors(self) -> MutBorrowedDFSPostorderIteratorWithAncestors<'a, Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS postorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                MutBorrowedDFSPostorderIteratorWithAncestors::new(root)
            }
        }
    }
}

impl<'a, Node> Iterator for MutBorrowedDFSPostorderIterator<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    type Item = Node::MutBorrowedValue;
    dfs_postorder_next!(get_value_and_children_iter_mut);
}

pub struct MutBorrowedDFSPostorderIteratorWithContext<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    root: Option<&'a mut Node>,
    traversal_stack: Vec<<Node::MutBorrowedChildren as IntoIterator>::IntoIter>,
    into_iterator_stack: Vec<Node::MutBorrowedChildren>,
    current_context: TreeContextMut<'a, Node>,
}

impl<'a, Node> MutBorrowedDFSPostorderIteratorWithContext<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    fn new(root: &'a mut Node) -> MutBorrowedDFSPostorderIteratorWithContext<'_, Node> {
        Self {
            root: Some(root),
            traversal_stack: Vec::new(),
            into_iterator_stack: Vec::new(),
            current_context: TreeContextMut::new(),
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
            item_stack: self.current_context.ancestors,
            old_traversal_stack: self.traversal_stack.into_iter().collect(),
            new_traversal_stack: Vec::new(),
        }
    }
}

impl<'a, Node> StreamingIterator for MutBorrowedDFSPostorderIteratorWithContext<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    type Item = TreeContextMut<'a, Node>;
    fn advance(&mut self) {
        let mut is_first_iteration = true;
        if let Some(next) = self.root.take() {
            let (value, children) = next.get_value_and_children_iter_mut();
            // ASSUMPTION: self.into_iterator_stack will always outlive self.traversal_stack.
            // If that assumption is not true, this code will cause Undefined Behavior.
            self.traversal_stack.push(
                unsafe { core::ptr::read(&children as *const Node::MutBorrowedChildren) }
                    .into_iter(),
            );
            self.current_context.ancestors.push(value);
            self.current_context.path.push(usize::MAX);
            self.into_iterator_stack.push(children);
            is_first_iteration = false;
        }

        if self.traversal_stack.len() > self.into_iterator_stack.len() {
            self.traversal_stack.pop();
        }

        loop {
            if let Some(top) = self.traversal_stack.last_mut() {
                if let Some(node) = top.next() {
                    // Path is not populated on the first pass over just the root node.
                    if let Some(last) = self.current_context.path.last_mut() {
                        *last = last.wrapping_add(1);
                    }

                    let (value, children) = node.get_value_and_children_iter_mut();
                    if is_first_iteration {
                        self.current_context.ancestors.pop();
                    }

                    // ASSUMPTION: self.into_iterator_stack will always outlive self.traversal_stack.
                    // If that assumption is not true, this code will cause Undefined Behavior.
                    self.traversal_stack.push(
                        unsafe { core::ptr::read(&children as *const Node::MutBorrowedChildren) }
                            .into_iter(),
                    );
                    self.current_context.ancestors.push(value);
                    self.current_context.path.push(usize::MAX);
                    self.into_iterator_stack.push(children);
                    is_first_iteration = false;
                    continue;
                }

                if self.current_context.ancestors.len() > self.traversal_stack.len() {
                    self.current_context.ancestors.pop();
                }

                if let Some(top) = self.into_iterator_stack.pop() {
                    self.current_context.children = MaybeUninit::new(top);
                }
                self.current_context.path.pop();
                return;
            } else {
                if let Some(top) = self.into_iterator_stack.pop() {
                    self.current_context.children = MaybeUninit::new(top);
                }
                self.current_context.ancestors.pop();
                self.current_context.path.pop();
                return;
            }
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

impl<'a, Node> StreamingIteratorMut for MutBorrowedDFSPostorderIteratorWithContext<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    get_mut_context!();
}

pub struct MutBorrowedDFSPostorderIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    root: Option<&'a mut Node>,
    item_stack: Vec<Node::MutBorrowedValue>,
    traversal_stack: Vec<<Node::MutBorrowedChildren as IntoIterator>::IntoIter>,
}

impl<'a, Node> MutBorrowedDFSPostorderIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    fn new(root: &'a mut Node) -> MutBorrowedDFSPostorderIteratorWithAncestors<'_, Node> {
        Self {
            root: Some(root),
            item_stack: Vec::new(),
            traversal_stack: Vec::new(),
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
            old_traversal_stack: self.traversal_stack.into_iter().collect(),
            new_traversal_stack: Vec::new(),
        }
    }
}

impl<'a, Node> StreamingIterator for MutBorrowedDFSPostorderIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    type Item = [Node::MutBorrowedValue];
    postorder_ancestors_streaming_iterator_impl!(get_value_and_children_iter_mut);
}

impl<'a, Node> StreamingIteratorMut for MutBorrowedDFSPostorderIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    get_mut_ancestors!();
}

pub struct MutBorrowedBinaryDFSPostorderIterator<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    root: Option<&'a mut Node>,
    item_stack: Vec<Node::MutBorrowedValue>,
    traversal_stack: Vec<BinaryChildren<&'a mut Node>>,
}

impl<'a, Node> MutBorrowedBinaryDFSPostorderIterator<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    pub(crate) fn new(root: &'a mut Node) -> MutBorrowedBinaryDFSPostorderIterator<'a, Node> {
        MutBorrowedBinaryDFSPostorderIterator {
            root: Some(root),
            item_stack: Vec::new(),
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
    pub fn attach_context(self) -> MutBorrowedBinaryDFSPostorderIteratorWithContext<'a, Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS postorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                MutBorrowedBinaryDFSPostorderIteratorWithContext::new(root)
            }
        }
    }

    #[doc = include_str!("../../doc_files/attach_ancestors.md")]
    pub fn attach_ancestors(self) -> MutBorrowedBinaryDFSPostorderIteratorWithAncestors<'a, Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFS postorder iterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                MutBorrowedBinaryDFSPostorderIteratorWithAncestors::new(root)
            }
        }
    }
}

impl<'a, Node> Iterator for MutBorrowedBinaryDFSPostorderIterator<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    type Item = Node::MutBorrowedValue;
    dfs_postorder_next!(get_value_and_children_iter_mut);
}

pub struct MutBorrowedBinaryDFSPostorderIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    root: Option<&'a mut Node>,
    item_stack: Vec<Node::MutBorrowedValue>,
    traversal_stack: Vec<<BinaryChildren<&'a mut Node> as IntoIterator>::IntoIter>,
}

impl<'a, Node> MutBorrowedBinaryDFSPostorderIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    fn new(root: &'a mut Node) -> MutBorrowedBinaryDFSPostorderIteratorWithAncestors<'_, Node> {
        Self {
            root: Some(root),
            item_stack: Vec::new(),
            traversal_stack: Vec::new(),
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
            old_traversal_stack: self.traversal_stack.into_iter().collect(),
            new_traversal_stack: Vec::new(),
        }
    }
}

impl<'a, Node> StreamingIterator for MutBorrowedBinaryDFSPostorderIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    type Item = [Node::MutBorrowedValue];
    postorder_ancestors_streaming_iterator_impl!(get_value_and_children_iter_mut);
}

impl<'a, Node> StreamingIteratorMut for MutBorrowedBinaryDFSPostorderIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    get_mut_ancestors!();
}
pub struct MutBorrowedBinaryDFSPostorderIteratorWithContext<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    root: Option<&'a mut Node>,
    traversal_stack: Vec<IntoIter<Option<&'a mut Node>, 2>>,
    current_context: BinaryTreeContextMut<'a, Node>,
    into_iterator_stack: Vec<[Option<&'a mut Node>; 2]>,
}

impl<'a, Node> MutBorrowedBinaryDFSPostorderIteratorWithContext<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    fn new(root: &'a mut Node) -> MutBorrowedBinaryDFSPostorderIteratorWithContext<'_, Node> {
        Self {
            root: Some(root),
            current_context: BinaryTreeContextMut::new(),
            traversal_stack: Vec::new(),
            into_iterator_stack: Vec::new(),
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
        todo!();
    }
}

impl<'a, Node> StreamingIterator for MutBorrowedBinaryDFSPostorderIteratorWithContext<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    type Item = BinaryTreeContextMut<'a, Node>;
    fn advance(&mut self) {
        let mut is_first_iteration = true;
        if let Some(next) = self.root.take() {
            let (value, children) = next.get_value_and_children_binary_iter_mut();
            // ASSUMPTION: self.into_iterator_stack will always outlive self.traversal_stack.
            // If that assumption is not true, this code will cause Undefined Behavior.
            self.traversal_stack.push(
                unsafe { core::ptr::read(&children as *const [Option<&'a mut Node>; 2]) }
                    .into_iter(),
            );
            self.current_context.ancestors.push(value);
            self.current_context.path.push(usize::MAX);
            self.into_iterator_stack.push(children);
            is_first_iteration = false;
        }

        if self.traversal_stack.len() > self.into_iterator_stack.len() {
            self.traversal_stack.pop();
        }

        'outer: loop {
            if let Some(top) = self.traversal_stack.last_mut() {
                while let Some(node) = top.next() {
                    // Path is not populated on the first pass over just the root node.
                    if let Some(last) = self.current_context.path.last_mut() {
                        *last = last.wrapping_add(1);
                    }

                    if let Some(node) = node {
                        let (value, children) = node.get_value_and_children_binary_iter_mut();
                        if is_first_iteration {
                            self.current_context.ancestors.pop();
                        }

                        // ASSUMPTION: self.into_iterator_stack will always outlive self.traversal_stack.
                        // If that assumption is not true, this code will cause Undefined Behavior.
                        self.traversal_stack.push(
                            unsafe {
                                core::ptr::read(&children as *const [Option<&'a mut Node>; 2])
                            }
                            .into_iter(),
                        );
                        self.current_context.ancestors.push(value);
                        self.current_context.path.push(usize::MAX);
                        self.into_iterator_stack.push(children);
                        is_first_iteration = false;
                        continue 'outer;
                    }
                }

                if self.current_context.ancestors.len() > self.traversal_stack.len() {
                    self.current_context.ancestors.pop();
                }

                if let Some(top) = self.into_iterator_stack.pop() {
                    self.current_context.children = MaybeUninit::new(top);
                }
                self.current_context.path.pop();
                return;
            } else {
                if let Some(top) = self.into_iterator_stack.pop() {
                    self.current_context.children = MaybeUninit::new(top);
                }
                self.current_context.ancestors.pop();
                self.current_context.path.pop();
                return;
            }
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
