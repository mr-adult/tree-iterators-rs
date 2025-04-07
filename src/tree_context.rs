use alloc::vec::Vec;

#[derive(Debug)]
pub struct TreeContext<Value, Children> {
    #[doc = include_str!("../doc_files/path.md")]
    pub(crate) path: Vec<usize>,

    #[doc = include_str!("../doc_files/ancestors_vec.md")]
    pub(crate) ancestors: Vec<Value>,

    #[doc = include_str!("../doc_files/tree_context_children.md")]
    pub(crate) children: Option<Children>,
}

impl<Value, Children> TreeContext<Value, Children> {
    pub(crate) fn new() -> Self {
        Self {
            path: Vec::new(),
            ancestors: Vec::new(),
            children: None,
        }
    }

    /// Gets the depth of the current node in the tree. This is zero-based,
    /// so the root node is at depth zero.
    ///
    /// Ex. given a tree like the following, the depths would be as labeled.
    /// ```text
    ///        0       <- depth: 0
    ///       / \
    ///      1   2     <- depth: 1
    ///     / \ / \
    ///    3  4 5  6   <- depth: 2
    ///           /
    ///          7     <- depth: 3
    ///           \
    ///            8   <- depth: 4
    ///           /
    ///          9     <- depth: 5
    ///           \
    ///           10   <- depth: 6
    /// ```
    pub fn depth(&self) -> usize {
        self.ancestors().len() - 1
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
        self.children.as_ref().unwrap()
    }

    #[doc = include_str!("../doc_files/tree_context_children.md")]
    pub fn children_mut(&mut self) -> &mut Children {
        // children should always be populated unless the iterator is in the middle of its .next() method.
        self.children.as_mut().unwrap()
    }
}
