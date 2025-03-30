use core::mem::MaybeUninit;

use alloc::vec::Vec;

use crate::prelude::{BorrowedTreeNode, MutBorrowedTreeNode, OwnedTreeNode};

#[derive(Debug)]
pub struct TreeContextRef<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    #[doc = include_str!("../doc_files/path.md")]
    pub(crate) path: Vec<usize>,

    #[doc = include_str!("../doc_files/ancestors_vec.md")]
    pub(crate) ancestors: Vec<Node::BorrowedValue>,

    #[doc = include_str!("../doc_files/tree_context_children.md")]
    pub(crate) children: MaybeUninit<Node::BorrowedChildren>,
}

impl<'a, Node> TreeContextRef<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    pub(crate) fn new() -> Self {
        Self {
            path: Vec::new(),
            ancestors: Vec::new(),
            children: MaybeUninit::uninit(),
        }
    }

    #[doc = include_str!("../doc_files/path.md")]
    pub fn path(&self) -> &[usize] {
        &self.path
    }

    #[doc = include_str!("../doc_files/ancestors_vec.md")]
    pub fn ancestors(&self) -> &[Node::BorrowedValue] {
        &self.ancestors
    }

    #[doc = include_str!("../doc_files/ancestors_vec.md")]
    pub fn ancestors_mut(&mut self) -> &mut [Node::BorrowedValue] {
        &mut self.ancestors
    }

    #[doc = include_str!("../doc_files/tree_context_children.md")]
    pub fn children(&self) -> &Node::BorrowedChildren {
        // children should always be populated unless the iterator is in the middle of its .next() method.
        unsafe { self.children.assume_init_ref() }
    }

    #[doc = include_str!("../doc_files/tree_context_children.md")]
    pub fn children_mut(&mut self) -> &mut Node::BorrowedChildren {
        // children should always be populated unless the iterator is in the middle of its .next() method.
        unsafe { self.children.assume_init_mut() }
    }
}

#[derive(Debug)]
pub struct TreeContextMut<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    #[doc = include_str!("../doc_files/path.md")]
    pub(crate) path: Vec<usize>,

    #[doc = include_str!("../doc_files/ancestors_vec.md")]
    pub(crate) ancestors: Vec<Node::MutBorrowedValue>,

    #[doc = include_str!("../doc_files/tree_context_children.md")]
    pub(crate) children: MaybeUninit<Node::MutBorrowedChildren>,
}

impl<'a, Node> TreeContextMut<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    pub(crate) fn new() -> Self {
        Self {
            path: Vec::new(),
            ancestors: Vec::new(),
            children: MaybeUninit::uninit(),
        }
    }

    #[doc = include_str!("../doc_files/path.md")]
    pub fn path(&self) -> &[usize] {
        &self.path
    }

    #[doc = include_str!("../doc_files/ancestors_vec.md")]
    pub fn ancestors(&self) -> &[Node::MutBorrowedValue] {
        &self.ancestors
    }

    #[doc = include_str!("../doc_files/ancestors_vec.md")]
    pub fn ancestors_mut(&mut self) -> &mut [Node::MutBorrowedValue] {
        &mut self.ancestors
    }

    #[doc = include_str!("../doc_files/tree_context_children.md")]
    pub fn children(&self) -> &Node::MutBorrowedChildren {
        // children should always be populated unless the iterator is in the middle of its .next() method.
        unsafe { self.children.assume_init_ref() }
    }

    #[doc = include_str!("../doc_files/tree_context_children.md")]
    pub fn children_mut(&mut self) -> &mut Node::MutBorrowedChildren {
        // children should always be populated unless the iterator is in the middle of its .next() method.
        unsafe { self.children.assume_init_mut() }
    }
}

#[derive(Debug)]
pub struct TreeContext<Node>
where
    Node: OwnedTreeNode,
{
    #[doc = include_str!("../doc_files/path.md")]
    pub(crate) path: Vec<usize>,

    #[doc = include_str!("../doc_files/ancestors_vec.md")]
    pub(crate) ancestors: Vec<Node::OwnedValue>,

    #[doc = include_str!("../doc_files/tree_context_children.md")]
    pub(crate) children: MaybeUninit<Node::OwnedChildren>,
}

impl<Node> TreeContext<Node>
where
    Node: OwnedTreeNode,
{
    pub(crate) fn new() -> Self {
        Self {
            path: Vec::new(),
            ancestors: Vec::new(),
            children: MaybeUninit::uninit(),
        }
    }

    #[doc = include_str!("../doc_files/path.md")]
    pub fn path(&self) -> &[usize] {
        &self.path
    }

    #[doc = include_str!("../doc_files/ancestors_vec.md")]
    pub fn ancestors(&self) -> &[Node::OwnedValue] {
        &self.ancestors
    }

    #[doc = include_str!("../doc_files/ancestors_vec.md")]
    pub fn ancestors_mut(&mut self) -> &mut [Node::OwnedValue] {
        &mut self.ancestors
    }

    #[doc = include_str!("../doc_files/tree_context_children.md")]
    pub fn children(&self) -> &Node::OwnedChildren {
        // children should always be populated unless the iterator is in the middle of its .next() method.
        unsafe { self.children.assume_init_ref() }
    }

    #[doc = include_str!("../doc_files/tree_context_children.md")]
    pub fn children_mut(&mut self) -> &mut Node::OwnedChildren {
        // children should always be populated unless the iterator is in the middle of its .next() method.
        unsafe { self.children.assume_init_mut() }
    }
}

#[derive(Debug)]
pub struct TreeContextNoChildren<Node>
where
    Node: OwnedTreeNode,
{
    #[doc = include_str!("../doc_files/path.md")]
    pub(crate) path: Vec<usize>,

    #[doc = include_str!("../doc_files/ancestors_vec.md")]
    pub(crate) ancestors: Vec<Node::OwnedValue>,
}

impl<Node> TreeContextNoChildren<Node>
where
    Node: OwnedTreeNode,
{
    pub(crate) fn new() -> Self {
        Self {
            path: Vec::new(),
            ancestors: Vec::new(),
        }
    }

    #[doc = include_str!("../doc_files/path.md")]
    pub fn path(&self) -> &[usize] {
        &self.path
    }

    #[doc = include_str!("../doc_files/ancestors_vec.md")]
    pub fn ancestors(&self) -> &[Node::OwnedValue] {
        &self.ancestors
    }

    #[doc = include_str!("../doc_files/ancestors_vec.md")]
    pub fn ancestors_mut(&mut self) -> &mut [Node::OwnedValue] {
        &mut self.ancestors
    }
}
