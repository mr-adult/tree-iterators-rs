use streaming_iterator::StreamingIterator;

use crate::prelude::BorrowedTreeNode;

pub struct BorrowedDFSPreorderIterator<'a, Node>
    where Node: BorrowedTreeNode<'a> {

    root: Option<&'a Node>,
    traversal_stack: Vec<Node::BorrowedChildren>,
}

impl<'a, Node> BorrowedDFSPreorderIterator<'a, Node> 
    where Node: BorrowedTreeNode<'a> {
        
    pub (crate) fn new(root: &'a Node) -> BorrowedDFSPreorderIterator<'a, Node> {
        BorrowedDFSPreorderIterator { 
            root: Some(root),
            traversal_stack: Vec::new()
        }
    }

    /// This method retrieves an iterator that can be used to perform
    /// Depth First Preorder searches of a tree.
    /// 
    /// A Depth First Preorder search is defined as:
    /// 
    /// A tree traversal that involves depth-first searching a tree 
    /// from the top down. this 
    /// traversal type would traverse the elements and yield slices in
    /// the following order. Note for each slice, the current node is
    /// at index slice.len() - 1, the root is at index 0 and all other 
    /// ancestors are found in between.
    /// - \[0\],
    /// - \[0, 1\],
    /// - \[0, 1, 3\],
    /// - \[0, 1, 4\],
    /// - \[0, 2\],
    /// - \[0, 2, 5\],
    /// - \[0, 2, 6\],
    /// - \[0, 2, 6, 7\],
    /// - \[0, 2, 6, 7, 8\],
    /// - \[0, 2, 6, 7, 8, 9\],
    /// - \[0, 2, 6, 7, 8, 9, 10\],
    /// 
    /// In this traversal, each node will only be traversed before any
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
    pub fn attach_ancestors(self) -> BorrowedDFSPreorderIteratorWithAncestors<'a, Node> {
        match self.root {
            None => panic!("Attempted to attach metadata to a DFSPostOrderIterator in the middle of a tree traversal. This is forbidden."),
            Some(root) => {
                BorrowedDFSPreorderIteratorWithAncestors::new(root)
            }
        }
    }

    fn pop_empty_iterators_until_move(&mut self) -> Option<&'a Node> {
        loop {
            let stack_len = self.traversal_stack.len();
            if stack_len == 0 { return None; }
            match self.traversal_stack.get_mut(stack_len - 1) {
                None => return None,
                Some(top) => {
                    match top.next() {
                        None => {
                            self.traversal_stack.pop();
                        }
                        Some(value) => {
                            return Some(value);
                        }
                    }
                }
            }
        }
    }
}

impl<'a, Node> Iterator for BorrowedDFSPreorderIterator<'a, Node> 
    where Node: BorrowedTreeNode<'a> {
    
    type Item = Node::BorrowedValue;
    
    fn next(&mut self) -> Option<Self::Item> {
        match std::mem::take(&mut self.root) {
            Some(next) => {
                let (value, children) = next.get_value_and_children_iter();
                match children {
                    None => {}
                    Some(children) => self.traversal_stack.push(children)
                }
                return Some(value);
            }
            None => {
                let next = self.pop_empty_iterators_until_move();
                match next {
                    None => return None,
                    Some(node) => {
                        let (value, children) = node.get_value_and_children_iter();
                        match  children {
                            None => {}
                            Some(children) => self.traversal_stack.push(children),
                        }
                        return Some(value);
                    }
                }
            }
        }
    }
}

pub struct BorrowedDFSPreorderIteratorWithAncestors<'a, Node>
    where Node: BorrowedTreeNode<'a> {

    root: Option<&'a Node>,
    traversal_stack: Vec<Node::BorrowedChildren>,
    item_stack: Vec<Node::BorrowedValue>,
}


impl<'a, Node> BorrowedDFSPreorderIteratorWithAncestors<'a, Node> 
    where Node: BorrowedTreeNode<'a> {
        
    pub (crate) fn new(root: &'a Node) -> BorrowedDFSPreorderIteratorWithAncestors<'a, Node> {
        BorrowedDFSPreorderIteratorWithAncestors { 
            root: Some(root),
            traversal_stack: Vec::new(),
            item_stack: Vec::new(),
        }
    }

    fn pop_empty_iterators_until_move(&mut self) -> Option<&'a Node> {
        loop {
            let stack_len = self.traversal_stack.len();
            if stack_len == 0 { return None; }
            match self.traversal_stack.get_mut(stack_len - 1) {
                None => return None,
                Some(top) => {
                    if self.item_stack.len() > stack_len {
                        self.item_stack.pop();
                    }
                    match top.next() {
                        None => {
                            self.traversal_stack.pop();
                            self.item_stack.pop();
                        }
                        Some(value) => {
                            return Some(value);
                        }
                    }
                }
            }
        }
    }
}

impl<'a, Node> StreamingIterator for BorrowedDFSPreorderIteratorWithAncestors<'a, Node> 
    where Node: BorrowedTreeNode<'a> {
    
    type Item = [Node::BorrowedValue];
    
    fn advance(&mut self) {
        match std::mem::take(&mut self.root) {
            Some(next) => {
                let (value, children) = next.get_value_and_children_iter();
                match children {
                    None => {}
                    Some(children) => self.traversal_stack.push(children)
                }

                self.item_stack.push(value);
                return;
            }
            None => {
                let next = self.pop_empty_iterators_until_move();
                match next {
                    None => return,
                    Some(node) => {
                        let (value, children) = node.get_value_and_children_iter();
                        match  children {
                            None => {}
                            Some(children) => self.traversal_stack.push(children),
                        }
                        self.item_stack.push(value);
                        return;
                    }
                }
            }
        }
    }

    fn get(&self) -> Option<&Self::Item> {
        if self.item_stack.len() > 0 {
            Some(&self.item_stack)
        } else {
            None
        }
    }
}
