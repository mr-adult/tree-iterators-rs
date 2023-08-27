use std::collections::VecDeque;
use streaming_iterator::StreamingIterator;

use crate::prelude::BorrowedTreeNode;

pub struct BorrowedBFSIterator<'a, Node> 
    where Node: BorrowedTreeNode<'a> {

    root: Option<&'a Node>,
    traversal_queue: VecDeque<Node::BorrowedChildren>
}

impl<'a, Node> BorrowedBFSIterator<'a, Node>
    where Node: BorrowedTreeNode<'a> {

    pub (crate) fn new(root: &'a Node) -> BorrowedBFSIterator<'a, Node> {
        BorrowedBFSIterator { 
            root: Some(root), 
            traversal_queue: VecDeque::new() 
        }
    }

    /// This method retrieves a streaming iterator that can be used to perform
    /// Breadth First searches of a tree. This converts the queue-based
    /// iterator into an iterative deepening iterator.
    /// 
    /// A Breadth First Search (BFS) is defined as:
    /// 
    /// A tree traversal that involves breadth-first searching a tree 
    /// from the top down. Given a tree of the following shape, this 
    /// traversal type would yield slices in the following order:
    /// \[0\], 
    /// \[0, 1\], 
    /// \[0, 2\], 
    /// \[0, 1, 3\], 
    /// \[0, 1, 4\], 
    /// \[0, 2, 5\], 
    /// \[0, 2, 5, 6\], 
    /// \[0, 2, 5, 6, 7\], 
    /// \[0, 2, 5, 6, 7, 8\], 
    /// \[0, 2, 5, 6, 7, 8, 9\], 
    /// \[0, 2, 5, 6, 7, 8, 9, 10\], 
    /// 
    /// In this traversal, we scan each level of the tree from left to
    /// right before going down to the next level.
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
    pub fn attach_ancestors(self) -> BorrowedBFSIteratorWithAncestors<'a, Node> {
        match self.root {
            None => panic!(),
            Some(root) => BorrowedBFSIteratorWithAncestors::new(root)
        }
    }
}

impl<'a, Node> Iterator for BorrowedBFSIterator<'a, Node> 
    where Node: BorrowedTreeNode<'a> {

    type Item = Node::BorrowedValue;
    fn next(&mut self) -> Option<Self::Item> {
        match std::mem::take(&mut self.root) {
            Some(root) => {
                let (value, children) = root.get_value_and_children_iter();
                match children {
                    None => {}
                    Some(children) => {
                        self.traversal_queue.push_back(children);
                    }
                }
                return Some(value);
            }
            None => {
                loop {
                    let next_queue_opt = self.traversal_queue.get_mut(0);
                    match next_queue_opt {
                        None => return None,
                        Some(next_queue) => {
                            match next_queue.next() {
                                None => {
                                    self.traversal_queue.pop_front();
                                    continue;
                                }
                                Some(next) => {
                                    let (value, children) = next.get_value_and_children_iter();
                                    match children {
                                        None => {}
                                        Some(children) => self.traversal_queue.push_back(children)                                   }
                                    return Some(value);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub struct BorrowedBFSIteratorWithAncestors<'a, Node> 
    where Node: BorrowedTreeNode<'a> {
    
    current_depth: usize,
    root: &'a Node,
    temp_root: Option<&'a Node>,
    item_stack: Vec<Node::BorrowedValue>,
    traversal_stack: Vec<Node::BorrowedChildren>,
}

impl<'a, Node> BorrowedBFSIteratorWithAncestors<'a, Node> 
    where Node: BorrowedTreeNode<'a> {

    fn new(root: &'a Node) -> BorrowedBFSIteratorWithAncestors<'a, Node> {
        BorrowedBFSIteratorWithAncestors {
            current_depth: 0,
            root: root,
            temp_root: Some(root),
            item_stack: Vec::new(),
            traversal_stack: Vec::new(),
        }
    }

    fn advance_dfs(&mut self) {
        match std::mem::take(&mut self.temp_root) {
            Some(next) => {
                let (value, children) = next.get_value_and_children_iter();
                match children {
                    None => {}
                    Some(children) => {
                        if self.traversal_stack.len() < self.current_depth {
                            self.traversal_stack.push(children)
                        }
                    }
                }

                if self.item_stack.len() <= self.current_depth {
                    self.item_stack.push(value);
                }
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
                            Some(children) => {
                                if self.traversal_stack.len() < self.current_depth { 
                                    self.traversal_stack.push(children);
                                }
                            }
                        }

                        if self.item_stack.len() <= self.current_depth {
                            self.item_stack.push(value);
                        }
                        return;
                    }
                }
            }
        }
    }

    fn pop_empty_iterators_until_move(&mut self) -> Option<&'a Node> {
        loop {
            let stack_len = self.traversal_stack.len();
            if stack_len == 0 { 
                if self.item_stack.len() > stack_len {
                    self.item_stack.pop();
                }
                return None; 
            }
            match self.traversal_stack.get_mut(stack_len - 1) {
                None => return None,
                Some(top) => {
                    match top.next() {
                        None => {
                            self.traversal_stack.pop();
                            self.item_stack.pop();
                        }
                        Some(value) => {
                            if self.item_stack.len() > stack_len {
                                self.item_stack.pop();
                            }
                            return Some(value);
                        }
                    }
                }
            }
        }
    }
}

impl<'a, Node> StreamingIterator for BorrowedBFSIteratorWithAncestors<'a, Node> 
    where Node: BorrowedTreeNode<'a> {

    type Item = [Node::BorrowedValue];

    fn advance(&mut self) {
        self.advance_dfs();
        if self.item_stack.len() == 0 {
            self.current_depth += 1;
        }
        let mut did_full_traveral = false;
        while self.item_stack.len() <= self.current_depth {
            if self.item_stack.len() == 0 {
                if did_full_traveral {
                    break;
                }
                self.temp_root = Some(self.root);
                did_full_traveral = true;
            }
            self.advance_dfs();
        }
    }

    fn get(&self) -> Option<&Self::Item> {
        if self.item_stack.len() > 0 {
            Some(self.item_stack.as_slice())
        } else {
            None
        }
    }
}