use streaming_iterator::StreamingIterator;
use crate::prelude::BorrowedTreeNode;

pub struct BorrowedDFSPostorderIterator<'a, Node> 
    where Node: BorrowedTreeNode<'a> {

    root: Option<&'a Node>,
    item_stack: Vec<Node::BorrowedValue>,
    traversal_stack: Vec<Node::BorrowedChildren>
}

impl<'a, Node> BorrowedDFSPostorderIterator<'a, Node> 
    where Node: BorrowedTreeNode<'a> {

    pub (crate) fn new(root: &'a Node) -> BorrowedDFSPostorderIterator<'a, Node> {
        BorrowedDFSPostorderIterator { 
            root: Some(root),
            item_stack: Vec::new(), 
            traversal_stack: Vec::new() 
        }
    }

    /// This method retrieves a streaming iterator that can be used to perform
    /// Depth First Postorder searches of a tree.
    /// 
    /// A Depth First Postorder search (referred to as DFS Postorder) 
    /// is defined as:
    /// 
    /// A tree traversal that involves depth-first searching a tree 
    /// from the bottom up. Given a tree of the following shape, this 
    /// traversal type would traverse the elements and yield slices in
    /// the following order. Note for each slice, the current node is
    /// at index slice.len() - 1, the root is at index 0 and all other 
    /// ancestors are found in between.
    /// - \[0, 1, 3\], 
    /// - \[0, 1, 4\], 
    /// - \[0, 1\], 
    /// - \[0, 2, 5\], 
    /// - \[0, 2, 6, 7, 8, 9, 10\], 
    /// - \[0, 2, 6, 7, 8, 9\], 
    /// - \[0, 3, 6, 7, 8\], 
    /// - \[0, 2, 6, 7\], 
    /// - \[0, 2, 6\], 
    /// - \[0, 2\], 
    /// - \[0\]
    /// 
    /// In this traversal, each node will only be traversed after all
    /// of its children have been traversed.
    /// -        0
    /// -       / \
    /// -      1   2
    /// -     / \ / \
    /// -    3  4 5  6
    /// -           /
    /// -          7
    /// -           \
    /// -            8
    /// -           /
    /// -          9
    /// -           \
    /// -           10
    ///
    /// More technical details:
    /// 
    /// This method attaches the ancestors of the node to the iterator.
    /// This operation transforms the iterator into a StreamingIterator,
    /// meaning that the values can no longer be directly save and used 
    /// across loop iterations. The references to the nodes themselves 
    /// are still valid across the entirety of the loop, but you must 
    /// extract them from their containing slice to reuse them. This
    /// will incure a performance penalty that this library does not
    /// assume you want.
    /// 
    /// Since this iterator is no longer a Rust Iterator, for loops will
    /// no longer work. See details on how to work around this in the 
    /// [streaming-iterator](https://crates.io/crates/streaming-iterator) crate.
    /// 
    pub fn attach_ancestors(self) -> BorrowedDFSPostorderIteratorWithAncestors<'a, Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFSPostOrderIterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                BorrowedDFSPostorderIteratorWithAncestors::new(root)
            }
        }
    }
}

impl<'a, Node> Iterator for BorrowedDFSPostorderIterator<'a, Node> 
    where Node: BorrowedTreeNode<'a> {

    type Item = Node::BorrowedValue;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match std::mem::take(&mut self.root) {
                Some(next) => {
                    let (value, children) = next.get_value_and_children_iter();
                    match children {
                        None => { return Some(value); }
                        Some(children) => {
                            self.traversal_stack.push(children);
                            self.item_stack.push(value);
                        }
                    }
                }
                None => {
                    loop {
                        let stack_len = self.traversal_stack.len();
                        if stack_len < 1 { return None; }
                        match self.traversal_stack.get_mut(stack_len - 1) {
                            None => return self.item_stack.pop(),
                            Some(next_iter) => {
                                match next_iter.next() {
                                    None => {
                                        self.traversal_stack.pop();
                                        return self.item_stack.pop();
                                    }
                                    Some(node) => {
                                        let (value, children) = node.get_value_and_children_iter();
                                        self.item_stack.push(value);
                                        match children {
                                            None => { return self.item_stack.pop(); }
                                            Some(children) => self.traversal_stack.push(children)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub struct BorrowedDFSPostorderIteratorWithAncestors<'a, Node>
    where Node: BorrowedTreeNode<'a> {

    root: Option<&'a Node>,
    item_stack: Vec<Node::BorrowedValue>,
    traversal_stack: Vec<Node::BorrowedChildren>,
}

impl<'a, Node> BorrowedDFSPostorderIteratorWithAncestors<'a, Node> 
    where Node: BorrowedTreeNode<'a> {

    fn new(root: &'a Node) -> BorrowedDFSPostorderIteratorWithAncestors<'_, Node> {
        Self {
            root: Some(root),
            item_stack: Vec::new(),
            traversal_stack: Vec::new(),
        }
    }
}

type TreeValueStack<T> = [T];

impl<'a, Node> StreamingIterator for BorrowedDFSPostorderIteratorWithAncestors<'a, Node> 
    where Node: BorrowedTreeNode<'a> {

    type Item = TreeValueStack<Node::BorrowedValue>;

    fn advance(&mut self) {
        let mut is_first_iteration = true;
        loop {
            match std::mem::take(&mut self.root) {
                Some(next) => {
                    let (value, children) = next.get_value_and_children_iter();
                    match children {
                        None => return,
                        Some(children) => {
                            self.traversal_stack.push(children);
                            self.item_stack.push(value);
                            is_first_iteration = false;
                        }
                    }
                }
                None => {
                    let mut pushed_another_iterator = false;
                    loop {
                        let stack_len = self.traversal_stack.len();
                        if stack_len < 1 { 
                            self.item_stack.pop();
                            return; 
                        }
                        match self.traversal_stack.get_mut(stack_len - 1) {
                            None => { 
                                self.item_stack.pop(); 
                                return; 
                            }
                            Some(next_iter) => {
                                match next_iter.next() {
                                    None => {
                                        if self.item_stack.len() > self.traversal_stack.len() {
                                            self.item_stack.pop(); 
                                        }
                                        self.traversal_stack.pop();
                                        return;
                                    }
                                    Some(node) => {
                                        let (value, children) = node.get_value_and_children_iter();
                                        match children {
                                            None => {
                                                if !pushed_another_iterator { self.item_stack.pop(); }
                                                self.item_stack.push(value);
                                                return;
                                            }
                                            Some(children) => {
                                                if is_first_iteration { self.item_stack.pop(); }
                                                self.traversal_stack.push(children);
                                                self.item_stack.push(value);
                                                pushed_another_iterator = true;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        is_first_iteration = false;
                    }
                }
            }
        }
    }

    fn get(&self) -> Option<&Self::Item> {
        if self.item_stack.len() > 0 {
            Some(&self.item_stack.as_slice())
        } else {
            None
        }
    }
}