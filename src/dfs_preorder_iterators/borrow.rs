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
