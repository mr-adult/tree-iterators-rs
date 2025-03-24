use alloc::vec::Vec;

use crate::prelude::{BorrowedTreeNode, MutBorrowedTreeNode, OwnedTreeNode};

#[derive(Clone, Debug)]
pub struct TreeContextRef<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    #[doc = include_str!("../doc_files/path.md")]
    pub(crate) path: Vec<usize>,

    #[doc = include_str!("../doc_files/ancestors_vec.md")]
    pub(crate) ancestors: Vec<Node::BorrowedValue>,

    #[doc = include_str!("../doc_files/tree_context_children.md")]
    pub(crate) children: Option<Node::BorrowedChildren>,
}

impl<'a, Node> TreeContextRef<'a, Node>
where
    Node: BorrowedTreeNode<'a>,
{
    pub(crate) fn new() -> Self {
        Self {
            path: Vec::new(),
            ancestors: Vec::new(),
            children: None,
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
        self.children
            .as_ref()
            .expect("There to always be children when a caller can get access to a TreeContextRef")
    }

    #[doc = include_str!("../doc_files/tree_context_children.md")]
    pub fn children_mut(&mut self) -> &mut Node::BorrowedChildren {
        self.children
            .as_mut()
            .expect("There to always be children when a caller can get access to a TreeContextRef")
    }
}

#[derive(Clone, Debug)]
pub struct TreeContextMut<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    #[doc = include_str!("../doc_files/path.md")]
    pub(crate) path: Vec<usize>,

    #[doc = include_str!("../doc_files/ancestors_vec.md")]
    pub(crate) ancestors: Vec<Node::MutBorrowedValue>,

    #[doc = include_str!("../doc_files/tree_context_children.md")]
    pub(crate) children: Option<Node::MutBorrowedChildren>,
}

impl<'a, Node> TreeContextMut<'a, Node>
where
    Node: MutBorrowedTreeNode<'a>,
{
    pub(crate) fn new() -> Self {
        Self {
            path: Vec::new(),
            ancestors: Vec::new(),
            children: None,
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
        self.children
            .as_ref()
            .expect("There to always be children when a caller can get access to a TreeContextRef")
    }

    #[doc = include_str!("../doc_files/tree_context_children.md")]
    pub fn children_mut(&mut self) -> &mut Node::MutBorrowedChildren {
        self.children
            .as_mut()
            .expect("There to always be children when a caller can get access to a TreeContextRef")
    }
}

#[derive(Clone, Debug)]
pub struct TreeContext<Node>
where
    Node: OwnedTreeNode,
{
    #[doc = include_str!("../doc_files/path.md")]
    pub(crate) path: Vec<usize>,

    #[doc = include_str!("../doc_files/ancestors_vec.md")]
    pub(crate) ancestors: Vec<Node::OwnedValue>,

    #[doc = include_str!("../doc_files/tree_context_children.md")]
    pub(crate) children: Option<Node::OwnedChildren>,
}

impl<Node> TreeContext<Node>
where
    Node: OwnedTreeNode,
{
    pub(crate) fn new() -> Self {
        Self {
            path: Vec::new(),
            ancestors: Vec::new(),
            children: None,
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
        self.children
            .as_ref()
            .expect("There to always be children when a caller can get access to a TreeContextRef")
    }

    #[doc = include_str!("../doc_files/tree_context_children.md")]
    pub fn children_mut(&mut self) -> &mut Node::OwnedChildren {
        self.children
            .as_mut()
            .expect("There to always be children when a caller can get access to a TreeContextRef")
    }
}
