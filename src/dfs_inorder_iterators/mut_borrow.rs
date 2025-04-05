use core::{mem::MaybeUninit, option::IntoIter};

use alloc::vec::Vec;
use streaming_iterator::{StreamingIterator, StreamingIteratorMut};

use crate::{
    leaves_iterators::{
        ancestors_depth_first::mut_borrow::MutBorrowedBinaryDFSLeavesPostorderIteratorWithAncestors,
        depth_first::mut_borrow::MutBorrowedBinaryLeavesIterator,
    },
    prelude::{BinaryTreeContextMut, MutBorrowedBinaryTreeNode},
};

use super::{
    dfs_inorder_ancestors_streaming_iterator_impl, dfs_inorder_next, get_mut_ancestors,
    get_mut_context, TraversalStatus,
};

pub struct MutBorrowedDFSInorderIterator<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    right_stack: Vec<Option<&'a mut Node>>,
    item_stack: Vec<Node::MutBorrowedValue>,
    moved: bool,
}

impl<'a, Node> MutBorrowedDFSInorderIterator<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    pub(crate) fn new(root: &'a mut Node) -> MutBorrowedDFSInorderIterator<'a, Node> {
        let mut right_stack = Vec::new();
        right_stack.push(Some(root));

        MutBorrowedDFSInorderIterator {
            right_stack,
            item_stack: Vec::new(),
            moved: false,
        }
    }

    #[doc = include_str!("../../doc_files/leaves.md")]
    pub fn leaves(self) -> MutBorrowedBinaryLeavesIterator<'a, Node, IntoIter<&'a mut Node>> {
        let mut traversal_stack_bottom = Vec::with_capacity(self.right_stack.capacity());
        for opt in self.right_stack {
            traversal_stack_bottom.push(opt.into_iter());
        }

        MutBorrowedBinaryLeavesIterator {
            root: None,
            traversal_stack_bottom: traversal_stack_bottom,
            traversal_stack_top: Vec::new(),
            item_stack: Vec::new(),
        }
    }

    #[doc = include_str!("../../doc_files/attach_context.md")]
    pub fn attach_context(mut self) -> MutBorrowedDFSInorderIteratorWithContext<'a, Node> {
        let root = self.right_stack.pop();
        match self.moved {
            true => panic!("Attempted to attach metadata to a BFS iterator in the middle of a tree traversal. This is forbidden."),
            false => MutBorrowedDFSInorderIteratorWithContext::new(root.unwrap().unwrap())
        }
    }

    #[doc = include_str!("../../doc_files/attach_ancestors.md")]
    pub fn attach_ancestors(mut self) -> MutBorrowedDFSInorderIteratorWithAncestors<'a, Node> {
        let root = self.right_stack.pop();
        match self.moved {
            true => panic!("Attempted to attach metadata to a BFS iterator in the middle of a tree traversal. This is forbidden."),
            false => MutBorrowedDFSInorderIteratorWithAncestors::new(root.unwrap().unwrap())
        }
    }
}

impl<'a, Node> Iterator for MutBorrowedDFSInorderIterator<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    type Item = Node::MutBorrowedValue;

    dfs_inorder_next!(get_value_and_children_binary_iter_mut);
}

pub struct MutBorrowedDFSInorderIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    right_stack: Vec<Option<&'a mut Node>>,
    item_stack: Vec<Node::MutBorrowedValue>,
    status_stack: Vec<TraversalStatus>,
}

impl<'a, Node> MutBorrowedDFSInorderIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    pub(crate) fn new(root: &'a mut Node) -> MutBorrowedDFSInorderIteratorWithAncestors<'a, Node> {
        let mut right_stack = Vec::new();
        right_stack.push(Some(root));

        MutBorrowedDFSInorderIteratorWithAncestors {
            right_stack,
            item_stack: Vec::new(),
            status_stack: Vec::new(),
        }
    }

    #[doc = include_str!("../../doc_files/ancestors_leaves.md")]
    pub fn leaves(
        mut self,
    ) -> MutBorrowedBinaryDFSLeavesPostorderIteratorWithAncestors<'a, Node, IntoIter<&'a mut Node>>
    {
        let root;
        let old_traversal_stack;

        if self.right_stack.len() == 1 {
            root = core::mem::take(self.right_stack.get_mut(0).unwrap());
            old_traversal_stack = Vec::new();
        } else {
            root = None;
            old_traversal_stack = self
                .right_stack
                .into_iter()
                .map(|opt| opt.into_iter())
                .collect();
        }

        MutBorrowedBinaryDFSLeavesPostorderIteratorWithAncestors {
            root,
            item_stack: self.item_stack,
            old_traversal_stack,
            new_traversal_stack: Vec::new(),
        }
    }
}

impl<'a, Node> StreamingIterator for MutBorrowedDFSInorderIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    type Item = [Node::MutBorrowedValue];

    dfs_inorder_ancestors_streaming_iterator_impl!(get_value_and_children_binary_iter_mut);
}

impl<'a, Node> StreamingIteratorMut for MutBorrowedDFSInorderIteratorWithAncestors<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    get_mut_ancestors!();
}

pub struct MutBorrowedDFSInorderIteratorWithContext<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    right_stack: Vec<Option<&'a mut Node>>,
    current_context: BinaryTreeContextMut<'a, Node>,
    into_iterator_stack: Vec<[Option<&'a mut Node>; 2]>,
    status_stack: Vec<TraversalStatus>,
}

impl<'a, Node> MutBorrowedDFSInorderIteratorWithContext<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    pub(crate) fn new(root: &'a mut Node) -> MutBorrowedDFSInorderIteratorWithContext<Node> {
        let mut right_stack = Vec::new();
        right_stack.push(Some(root));

        let context = BinaryTreeContextMut::new();

        Self {
            right_stack,
            current_context: context,
            into_iterator_stack: Vec::new(),
            status_stack: Vec::new(),
        }
    }
}

impl<'a, Node> StreamingIterator for MutBorrowedDFSInorderIteratorWithContext<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    type Item = BinaryTreeContextMut<'a, Node>;

    fn advance(&mut self) {
        let mut current = None;
        while current.is_none() {
            if let Some(last_status) = self.status_stack.last_mut() {
                match last_status {
                    TraversalStatus::WentRight => {
                        self.current_context.ancestors.pop();
                        self.current_context.path.pop();
                        self.status_stack.pop();
                        continue;
                    }
                    TraversalStatus::WentLeft => {
                        *last_status = TraversalStatus::ReturnedSelf;
                        self.current_context.children =
                            MaybeUninit::new(self.into_iterator_stack.pop().unwrap());
                        return;
                    }
                    TraversalStatus::ReturnedSelf => {
                        *last_status = TraversalStatus::WentRight;
                    }
                }
            }

            if let Some(top_of_right_stack) = self.right_stack.pop() {
                current = top_of_right_stack;
                continue;
            } else {
                self.current_context.ancestors.clear();
                return;
            }
        }

        while let Some(current_val) = current {
            let (value, children) = current_val.get_value_and_children_binary_iter_mut();

            self.right_stack
                .push(unsafe { core::ptr::read(&children[1] as *const Option<&'a mut Node>) });
            let left = unsafe { core::ptr::read(&children[0] as *const Option<&'a mut Node>) };
            self.into_iterator_stack.push(children);

            self.current_context.ancestors.push(value);
            match self.status_stack.last() {
                None => {}
                Some(TraversalStatus::WentLeft | TraversalStatus::ReturnedSelf) => {
                    self.current_context.path.push(0)
                }
                Some(TraversalStatus::WentRight) => self.current_context.path.push(1),
            }
            self.status_stack.push(TraversalStatus::WentLeft);
            current = left;
        }

        let status_stack_len = self.status_stack.len();
        self.status_stack[status_stack_len - 1] = TraversalStatus::ReturnedSelf;

        self.current_context.children = MaybeUninit::new(
            self.into_iterator_stack
                .pop()
                .expect("There to be a children IntoIterator"),
        );
    }

    fn get(&self) -> Option<&Self::Item> {
        if self.current_context.ancestors.is_empty() {
            None
        } else {
            Some(&self.current_context)
        }
    }
}

impl<'a, Node> StreamingIteratorMut for MutBorrowedDFSInorderIteratorWithContext<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    get_mut_context!();
}
