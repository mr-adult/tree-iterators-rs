use crate::prelude::OwnedTreeNode;

pub struct OwnedDFSPreorderIterator<Node>
    where Node: OwnedTreeNode {

    root: Option<Node>,
    traversal_stack: Vec<Node::OwnedChildren>,
}

impl<Node> OwnedDFSPreorderIterator<Node> 
    where Node: OwnedTreeNode {
        
    pub (crate) fn new(root: Node) -> OwnedDFSPreorderIterator<Node> {
        OwnedDFSPreorderIterator { 
            root: Some(root),
            traversal_stack: Vec::new()
        }
    }

    fn pop_empty_iterators_until_move(&mut self) -> Option<Node> {
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

impl<Node> Iterator for OwnedDFSPreorderIterator<Node> 
    where Node: OwnedTreeNode {
    
    type Item = Node::OwnedValue;
    
    fn next(&mut self) -> Option<Self::Item> {
        match std::mem::take(&mut self.root) {
            Some(next) => {
                let (value, children) = next.get_value_and_children();
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
                        let (value, children) = node.get_value_and_children();
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