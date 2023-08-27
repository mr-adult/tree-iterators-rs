use streaming_iterator::StreamingIterator;
use crate::prelude::MutBorrowedTreeNode;

pub struct MutBorrowedDFSPostorderIterator<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {

    root: Option<&'a mut Node>,
    item_stack: Vec<Node::MutBorrowedValue>,
    traversal_stack: Vec<Node::MutBorrowedChildren>
}

impl<'a, Node> MutBorrowedDFSPostorderIterator<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {

    pub (crate) fn new(root: &'a mut Node) -> MutBorrowedDFSPostorderIterator<'a, Node> {
        MutBorrowedDFSPostorderIterator { 
            root: Some(root),
            item_stack: Vec::new(), 
            traversal_stack: Vec::new() 
        }
    }

    /// Attaches the ancestors of the node to the iterator.
    /// This operation transforms the iterator into a StreamingIterator,
    /// meaning that the values can no longer be directly save and used 
    /// across loop iterations. The references are still valid, but they
    /// must be extracted from their containing slice to reuse them.
    /// 
    /// You will be sacrificing the use of for loops, as this iterator is
    /// no longer a native rust iterator and must be supported separately.
    /// For more information, see the streaming-iterator crate on crates.io.
    /// 
    /// This method retrieves a streaming iterable that can be used to perform
    /// Depth First Postorder searches of a tree.
    /// 
    /// A Depth First Postorder search (referred to as DFS Postorder) 
    /// is defined as:
    /// 
    /// A tree traversal that involves depth-first searching a tree 
    /// from the bottom up. Given a tree of the following shape, this 
    /// traversal type would traverse the elements and yield slices in
    /// the following order: 
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
    pub fn attach_ancestors(self) -> MutBorrowedDFSPostorderIteratorWithAncestors<'a, Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFSPostOrderIterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                MutBorrowedDFSPostorderIteratorWithAncestors::new(root)
            }
        }
    }
}

impl<'a, Node> Iterator for MutBorrowedDFSPostorderIterator<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {

    type Item = Node::MutBorrowedValue;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match std::mem::take(&mut self.root) {
                Some(next) => {
                    let (value, children) = next.get_value_and_children_iter_mut();
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
                                        let (value, children) = node.get_value_and_children_iter_mut();
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

pub struct MutBorrowedDFSPostorderIteratorWithAncestors<'a, Node>
    where Node: MutBorrowedTreeNode<'a> {

    root: Option<&'a mut Node>,
    item_stack: Vec<Node::MutBorrowedValue>,
    traversal_stack: Vec<Node::MutBorrowedChildren>,
}

impl<'a, Node> MutBorrowedDFSPostorderIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {

    fn new(root: &'a mut Node) -> MutBorrowedDFSPostorderIteratorWithAncestors<'_, Node> {
        Self {
            root: Some(root),
            item_stack: Vec::new(),
            traversal_stack: Vec::new(),
        }
    }
}

type TreeValueStack<T> = [T];

impl<'a, Node> StreamingIterator for MutBorrowedDFSPostorderIteratorWithAncestors<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {

    type Item = TreeValueStack<Node::MutBorrowedValue>;

    fn advance(&mut self) {
        let mut is_first_iteration = true;
        loop {
            match std::mem::take(&mut self.root) {
                Some(next) => {
                    let (value, children) = next.get_value_and_children_iter_mut();
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
                                        let (value, children) = node.get_value_and_children_iter_mut();
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