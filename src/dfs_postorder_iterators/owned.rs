use crate::prelude::OwnedTreeNode;

pub struct OwnedDFSPostorderIterator<Node> 
    where Node: OwnedTreeNode {

    root: Option<Node>,
    item_stack: Vec<Node::OwnedValue>,
    traversal_stack: Vec<Node::OwnedChildren>
}

impl<Node> OwnedDFSPostorderIterator<Node> 
    where Node: OwnedTreeNode {

    pub (crate) fn new(root: Node) -> OwnedDFSPostorderIterator<Node> {
        OwnedDFSPostorderIterator { 
            root: Some(root),
            item_stack: Vec::new(), 
            traversal_stack: Vec::new() 
        }
    }
}

impl<Node> Iterator for OwnedDFSPostorderIterator<Node> 
    where Node: OwnedTreeNode {

    type Item = Node::OwnedValue;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match std::mem::take(&mut self.root) {
                Some(next) => {
                    let (value, children) = next.get_value_and_children();
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
                                        let (value, children) = node.get_value_and_children();
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