use core::option::IntoIter;

use alloc::vec::Vec;
use streaming_iterator::StreamingIterator;

use crate::{
    leaves_iterators::{
        ancestors_depth_first::borrow::BorrowedBinaryDFSLeavesPostorderIteratorWithAncestors,
        depth_first::borrow::BorrowedBinaryLeavesIterator,
    },
    prelude::{BorrowedBinaryTreeNode, TreeContext},
};

use super::{dfs_inorder_ancestors_streaming_iterator_impl, dfs_inorder_next, TraversalStatus};

crate::collection_iterators::borrowed_collection_iterator_impl!(
    BorrowedDFSInorderCollectionIterator,
    BorrowedDFSInorderIterator,
    BorrowedBinaryTreeNode
);

impl<'a, IntoIter, Node> BorrowedDFSInorderCollectionIterator<'a, IntoIter, Node>
where
    IntoIter: IntoIterator<Item = &'a Node>,
    Node: BorrowedBinaryTreeNode<'a>,
{
    #[doc = include_str!("../../doc_files/collection_attach_context.md")]
    pub fn attach_context(
        self,
    ) -> BorrowedDFSInorderCollectionIteratorWithContext<'a, IntoIter, Node> {
        BorrowedDFSInorderCollectionIteratorWithContext::new(self)
    }
}

crate::collection_iterators::borrowed_binary_collection_context_iterator_impl!(
    BorrowedDFSInorderCollectionIteratorWithContext,
    BorrowedDFSInorderIteratorWithContext,
    BorrowedDFSInorderCollectionIterator
);

pub struct BorrowedDFSInorderIterator<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    right_stack: Vec<Option<&'a Node>>,
    item_stack: Vec<Node::BorrowedValue>,
    moved: bool,
}

impl<'a, Node> BorrowedDFSInorderIterator<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    pub(crate) fn new(root: &'a Node) -> BorrowedDFSInorderIterator<'a, Node> {
        let mut right_stack = Vec::new();
        right_stack.push(Some(root));

        BorrowedDFSInorderIterator {
            right_stack,
            item_stack: Vec::new(),
            moved: false,
        }
    }

    #[doc = include_str!("../../doc_files/leaves.md")]
    pub fn leaves(self) -> BorrowedBinaryLeavesIterator<'a, Node, IntoIter<&'a Node>> {
        let mut traversal_stack_bottom = Vec::with_capacity(self.right_stack.capacity());
        for opt in self.right_stack {
            traversal_stack_bottom.push(opt.into_iter());
        }

        BorrowedBinaryLeavesIterator {
            root: None,
            traversal_stack_bottom: traversal_stack_bottom,
            traversal_stack_top: Vec::new(),
            item_stack: Vec::new(),
        }
    }

    #[doc = include_str!("../../doc_files/attach_context.md")]
    pub fn attach_context(mut self) -> BorrowedDFSInorderIteratorWithContext<'a, Node> {
        let root = self.right_stack.pop();
        match self.moved {
            true => panic!("Attempted to attach metadata to a BFS iterator in the middle of a tree traversal. This is forbidden."),
            false => BorrowedDFSInorderIteratorWithContext::new(root.unwrap().unwrap(), Vec::new())
        }
    }

    #[doc = include_str!("../../doc_files/attach_ancestors.md")]
    pub fn attach_ancestors(mut self) -> BorrowedDFSInorderIteratorWithAncestors<'a, Node> {
        let root = self.right_stack.pop();
        match self.moved {
            true => panic!("Attempted to attach metadata to a BFS iterator in the middle of a tree traversal. This is forbidden."),
            false => BorrowedDFSInorderIteratorWithAncestors::new(root.unwrap().unwrap())
        }
    }
}

impl<'a, Node> Iterator for BorrowedDFSInorderIterator<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    type Item = Node::BorrowedValue;

    dfs_inorder_next!(get_value_and_children_binary_iter);
}

pub struct BorrowedDFSInorderIteratorWithAncestors<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    right_stack: Vec<Option<&'a Node>>,
    item_stack: Vec<Node::BorrowedValue>,
    status_stack: Vec<TraversalStatus>,
}

impl<'a, Node> BorrowedDFSInorderIteratorWithAncestors<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    pub(crate) fn new(root: &'a Node) -> BorrowedDFSInorderIteratorWithAncestors<'a, Node> {
        let mut right_stack = Vec::new();
        right_stack.push(Some(root));

        BorrowedDFSInorderIteratorWithAncestors {
            right_stack,
            item_stack: Vec::new(),
            status_stack: Vec::new(),
        }
    }

    #[doc = include_str!("../../doc_files/ancestors_leaves.md")]
    pub fn leaves(
        mut self,
    ) -> BorrowedBinaryDFSLeavesPostorderIteratorWithAncestors<'a, Node, IntoIter<&'a Node>> {
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

        BorrowedBinaryDFSLeavesPostorderIteratorWithAncestors {
            root,
            item_stack: self.item_stack,
            old_traversal_stack,
            new_traversal_stack: Vec::new(),
        }
    }
}

impl<'a, Node> StreamingIterator for BorrowedDFSInorderIteratorWithAncestors<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    type Item = [Node::BorrowedValue];
    dfs_inorder_ancestors_streaming_iterator_impl!(get_value_and_children_binary_iter);
}

pub struct BorrowedDFSInorderIteratorWithContext<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    right_stack: Vec<Option<&'a Node>>,
    current_context: TreeContext<Node::BorrowedValue, [Option<&'a Node>; 2]>,
    into_iterator_stack: Vec<[Option<&'a Node>; 2]>,
    status_stack: Vec<TraversalStatus>,
}

impl<'a, Node> BorrowedDFSInorderIteratorWithContext<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    pub(crate) fn new(
        root: &'a Node,
        path: Vec<usize>,
    ) -> BorrowedDFSInorderIteratorWithContext<'a, Node> {
        let mut right_stack = Vec::new();
        right_stack.push(Some(root));

        let context = TreeContext {
            path,
            ancestors: Vec::new(),
            children: None,
        };

        Self {
            right_stack,
            current_context: context,
            into_iterator_stack: Vec::new(),
            status_stack: Vec::new(),
        }
    }
}

impl<'a, Node> StreamingIterator for BorrowedDFSInorderIteratorWithContext<'a, Node>
where
    Node: BorrowedBinaryTreeNode<'a>,
{
    type Item = TreeContext<Node::BorrowedValue, [Option<&'a Node>; 2]>;

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
                            Some(self.into_iterator_stack.pop().unwrap());
                        return;
                    }
                    TraversalStatus::ReturnedSelf => *last_status = TraversalStatus::WentRight,
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
            let (value, children) = current_val.get_value_and_children_binary_iter();

            self.right_stack
                .push(unsafe { core::ptr::read(&children[1] as *const Option<&'a Node>) });
            let left = unsafe { core::ptr::read(&children[0] as *const Option<&'a Node>) };
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

        self.current_context.children = Some(
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
