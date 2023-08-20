use std::collections::VecDeque;
use crate::prelude::OwnedTreeNode;

pub struct OwnedBFSIterator<Node> 
    where Node: OwnedTreeNode {

    root: Option<Node>,
    traversal_queue: VecDeque<Node::OwnedChildren>
}

impl<Node> OwnedBFSIterator<Node>
    where Node: OwnedTreeNode {

    pub (crate) fn new(root: Node) -> OwnedBFSIterator<Node> {
        OwnedBFSIterator { 
            root: Some(root), 
            traversal_queue: VecDeque::new() 
        }
    }
}

impl<Node> Iterator for OwnedBFSIterator<Node> 
    where Node: OwnedTreeNode {

    type Item = Node::OwnedValue;
    fn next(&mut self) -> Option<Self::Item> {
        match std::mem::take(&mut self.root) {
            Some(root) => {
                let (value, children) = root.get_value_and_children();
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
                                    let (value, children) = next.get_value_and_children();
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