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