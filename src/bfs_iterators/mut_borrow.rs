use std::collections::VecDeque;
use crate::prelude::MutBorrowedTreeNode;

pub struct MutBorrowedBFSIterator<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {

    root: Option<&'a mut Node>,
    traversal_queue: VecDeque<Node::MutBorrowedChildren>
}

impl<'a, Node> MutBorrowedBFSIterator<'a, Node>
    where Node: MutBorrowedTreeNode<'a> {

    pub (crate) fn new(root: &'a mut Node) -> MutBorrowedBFSIterator<'a, Node> {
        MutBorrowedBFSIterator { 
            root: Some(root), 
            traversal_queue: VecDeque::new() 
        }
    }
}

impl<'a, Node> Iterator for MutBorrowedBFSIterator<'a, Node> 
    where Node: MutBorrowedTreeNode<'a> {

    type Item = Node::MutBorrowedValue;
    fn next(&mut self) -> Option<Self::Item> {
        match std::mem::take(&mut self.root) {
            Some(root) => {
                let (value, children) = root.get_value_and_children_iter_mut();
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
                                    let (value, children) = next.get_value_and_children_iter_mut();
                                    match children {
                                        None => {}
                                        Some(children) => self.traversal_queue.push_back(children)
                                    }
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