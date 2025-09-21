use core::array::IntoIter;

use crate::{
    leaves_iterators::{
        ancestors_depth_first::mut_borrow::{
            MutBorrowedBinaryDFSLeavesPostorderIteratorWithAncestors,
            MutBorrowedDFSLeavesPostorderIteratorWithAncestors,
        },
        depth_first::mut_borrow::{MutBorrowedBinaryLeavesIterator, MutBorrowedLeavesIterator},
    },
    prelude::{BinaryChildren, MutBorrowedBinaryTreeNode, MutBorrowedTreeNode, TreeContext},
};
use alloc::vec::Vec;
use streaming_iterator::{StreamingIterator, StreamingIteratorMut};

use super::{
    dfs_postorder_next, get_mut_ancestors, get_mut_context,
    postorder_ancestors_streaming_iterator_impl,
};

crate::collection_iterators::mut_borrowed_collection_iterator_impl!(
    MutBorrowedDFSPostorderCollectionIterator,
    MutBorrowedDFSPostorderIterator,
    MutBorrowedTreeNode
);

impl<'a, IntoIter, Node> MutBorrowedDFSPostorderCollectionIterator<'a, IntoIter, Node>
where
    IntoIter: IntoIterator<Item = &'a mut Node>,
    Node: MutBorrowedTreeNode<'a>,
{
    #[doc = include_str!("../../doc_files/collection_attach_context.md")]
    pub fn attach_context(
        self,
    ) -> MutBorrowedDFSPostorderCollectionIteratorWithContext<'a, IntoIter, Node> {
        MutBorrowedDFSPostorderCollectionIteratorWithContext::new(self)
    }
}

crate::collection_iterators::mut_borrowed_collection_context_iterator_impl!(
    MutBorrowedDFSPostorderCollectionIteratorWithContext,
    MutBorrowedDFSPostorderIteratorWithContext,
    MutBorrowedDFSPostorderCollectionIterator
);

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
                MutBorrowedDFSPostorderIteratorWithContext::new(root, Vec::new())
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
    current_context: TreeContext<Node::MutBorrowedValue, Node::MutBorrowedChildren>,
}

impl<'a, Node> MutBorrowedDFSPostorderIteratorWithContext<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    fn new(
        root: &'a mut Node,
        path: Vec<usize>,
    ) -> MutBorrowedDFSPostorderIteratorWithContext<'a, Node> {
        Self {
            root: Some(root),
            traversal_stack: Vec::new(),
            into_iterator_stack: Vec::new(),
            current_context: TreeContext {
                path,
                ancestors: Vec::new(),
                children: None,
            },
        }
    }
}

impl<'a, Node> StreamingIterator for MutBorrowedDFSPostorderIteratorWithContext<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    type Item = TreeContext<Node::MutBorrowedValue, Node::MutBorrowedChildren>;
    fn advance(&mut self) {
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

                    let (value, children) = node.get_value_and_children_iter_mut();

                    // ASSUMPTION: self.into_iterator_stack will always outlive self.traversal_stack.
                    // If that assumption is not true, this code will cause Undefined Behavior.
                    self.traversal_stack.push(
                        unsafe { core::ptr::read(&children as *const Node::MutBorrowedChildren) }
                            .into_iter(),
                    );
                    self.current_context.ancestors.push(value);
                    self.current_context.path.push(usize::MAX);
                    self.into_iterator_stack.push(children);
                    continue;
                }
            }

            self.current_context.children = Some(
                self.into_iterator_stack
                    .pop()
                    .expect("There to be a children IntoIterator"),
            );
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
    fn new(root: &'a mut Node) -> MutBorrowedDFSPostorderIteratorWithAncestors<'a, Node> {
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

crate::collection_iterators::mut_borrowed_collection_iterator_impl!(
    MutBorrowedBinaryDFSPostorderCollectionIterator,
    MutBorrowedBinaryDFSPostorderIterator,
    MutBorrowedBinaryTreeNode
);

impl<'a, IntoIter, Node> MutBorrowedBinaryDFSPostorderCollectionIterator<'a, IntoIter, Node>
where
    IntoIter: IntoIterator<Item = &'a mut Node>,
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    #[doc = include_str!("../../doc_files/collection_attach_context.md")]
    pub fn attach_context(
        self,
    ) -> MutBorrowedBinaryDFSPostorderCollectionIteratorWithContext<'a, IntoIter, Node> {
        MutBorrowedBinaryDFSPostorderCollectionIteratorWithContext::new(self)
    }
}

crate::collection_iterators::mut_borrowed_binary_collection_context_iterator_impl!(
    MutBorrowedBinaryDFSPostorderCollectionIteratorWithContext,
    MutBorrowedBinaryDFSPostorderIteratorWithContext,
    MutBorrowedBinaryDFSPostorderCollectionIterator
);

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
                MutBorrowedBinaryDFSPostorderIteratorWithContext::new(root, Vec::new())
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
    fn new(root: &'a mut Node) -> MutBorrowedBinaryDFSPostorderIteratorWithAncestors<'a, Node> {
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
    traversal_stack: Vec<IntoIter<Option<*mut Node>, 2>>,
    current_context: TreeContext<Node::MutBorrowedValue, [Option<&'a mut Node>; 2]>,
    into_iterator_stack: Vec<[Option<*mut Node>; 2]>,
}

impl<'a, Node> MutBorrowedBinaryDFSPostorderIteratorWithContext<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    fn new(
        root: &'a mut Node,
        path: Vec<usize>,
    ) -> MutBorrowedBinaryDFSPostorderIteratorWithContext<'a, Node> {
        Self {
            root: Some(root),
            current_context: TreeContext {
                path,
                ancestors: Vec::new(),
                children: None,
            },
            traversal_stack: Vec::new(),
            into_iterator_stack: Vec::new(),
        }
    }
}

impl<'a, Node> StreamingIterator for MutBorrowedBinaryDFSPostorderIteratorWithContext<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    type Item = TreeContext<Node::MutBorrowedValue, [Option<&'a mut Node>; 2]>;
    fn advance(&mut self) {
        self.current_context.children = None;

        if let Some(next) = self.root.take() {
            let (value, mut children) = next.get_value_and_children_binary_iter_mut();

            let left = children[0].as_mut().map(|val| *val as *mut Node);
            let right = children[1].as_mut().map(|val| *val as *mut Node);

            // ASSUMPTION: self.into_iterator_stack will always outlive self.traversal_stack.
            // If that assumption is not true, this code will cause Undefined Behavior.
            self.traversal_stack
                .push([left.clone(), right.clone()].into_iter());
            self.into_iterator_stack.push([left, right]);

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
                for node in top.by_ref() {
                    let last = self
                        .current_context
                        .path
                        .last_mut()
                        .expect("There to be a path unless we are on the root element");
                    *last = last.wrapping_add(1);

                    if let Some(node) = node {
                        let (value, mut children) =
                            unsafe { &mut *node }.get_value_and_children_binary_iter_mut();

                        let left = children[0].as_mut().map(|val| *val as *mut Node);
                        let right = children[1].as_mut().map(|val| *val as *mut Node);

                        self.traversal_stack
                            .push([left.clone(), right.clone()].into_iter());
                        self.into_iterator_stack.push([left, right]);

                        // ASSUMPTION: self.into_iterator_stack will always outlive self.traversal_stack.
                        // If that assumption is not true, this code will cause Undefined Behavior.
                        self.current_context.ancestors.push(value);
                        self.current_context.path.push(usize::MAX);
                        continue 'outer;
                    }
                }
            }

            self.current_context.children = Some(
                self.into_iterator_stack
                    .pop()
                    .expect("There to be a children IntoIterator")
                    .map(|opt| opt.map(|val| unsafe { &mut *val })),
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

impl<'a, Node> StreamingIteratorMut for MutBorrowedBinaryDFSPostorderIteratorWithContext<'a, Node>
where
    Node: MutBorrowedBinaryTreeNode<'a>,
{
    fn get_mut(&mut self) -> Option<&mut Self::Item> {
        if self.current_context.ancestors.is_empty() {
            None
        } else {
            Some(&mut self.current_context)
        }
    }
}
