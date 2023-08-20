use std::collections::VecDeque;
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