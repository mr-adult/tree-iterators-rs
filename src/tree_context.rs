use core::mem::MaybeUninit;

use alloc::vec::Vec;

#[derive(Debug)]
pub struct TreeContext<Value, Children> {
    #[doc = include_str!("../doc_files/path.md")]
    pub(crate) path: Vec<usize>,

    #[doc = include_str!("../doc_files/ancestors_vec.md")]
    pub(crate) ancestors: Vec<Value>,

    #[doc = include_str!("../doc_files/tree_context_children.md")]
    pub(crate) children: MaybeUninit<Children>,
}

impl<Value, Children> TreeContext<Value, Children> {
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
    pub fn ancestors(&self) -> &[Value] {
        &self.ancestors
    }

    #[doc = include_str!("../doc_files/ancestors_vec.md")]
    pub fn ancestors_mut(&mut self) -> &mut [Value] {
        &mut self.ancestors
    }

    #[doc = include_str!("../doc_files/tree_context_children.md")]
    pub fn children(&self) -> &Children {
        // children should always be populated unless the iterator is in the middle of its .next() method.
        unsafe { self.children.assume_init_ref() }
    }

    #[doc = include_str!("../doc_files/tree_context_children.md")]
    pub fn children_mut(&mut self) -> &mut Children {
        // children should always be populated unless the iterator is in the middle of its .next() method.
        unsafe { self.children.assume_init_mut() }
    }
}
